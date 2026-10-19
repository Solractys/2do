use std::io::{self, Read, Write};
use std::mem::MaybeUninit;

pub struct Terminal {
    original: libc::termios,
}

impl Terminal {
    pub fn enter() -> io::Result<Self> {
        if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
            return Err(io::Error::other("a entrada não é um terminal"));
        }
        let mut original = MaybeUninit::<libc::termios>::uninit();
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, original.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }
        let original = unsafe { original.assume_init() };
        let mut raw = original;
        unsafe { libc::cfmakeraw(&mut raw) };
        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &raw) } != 0 {
            return Err(io::Error::last_os_error());
        }
        print!("\x1b[?1049h\x1b[?25l");
        io::stdout().flush()?;
        Ok(Self { original })
    }

    pub fn size(&self) -> (usize, usize) {
        let mut size = MaybeUninit::<libc::winsize>::zeroed();
        if unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, size.as_mut_ptr()) } == 0 {
            let size = unsafe { size.assume_init() };
            return (size.ws_col as usize, size.ws_row as usize);
        }
        (80, 24)
    }

    pub fn read_key(&self) -> io::Result<Key> {
        let mut byte = [0_u8];
        io::stdin().read_exact(&mut byte)?;
        if byte[0] == b'\r' || byte[0] == b'\n' {
            return Ok(Key::Enter);
        }
        if byte[0] == 8 || byte[0] == 127 {
            return Ok(Key::Backspace);
        }
        if byte[0] != 0x1b {
            let length = match byte[0] {
                0x00..=0x7f => 1,
                0xc0..=0xdf => 2,
                0xe0..=0xef => 3,
                _ => 4,
            };
            let mut bytes = [0_u8; 4];
            bytes[0] = byte[0];
            if length > 1 {
                io::stdin().read_exact(&mut bytes[1..length])?;
            }
            let character = std::str::from_utf8(&bytes[..length])
                .ok()
                .and_then(|text| text.chars().next())
                .unwrap_or('�');
            return Ok(Key::Char(character));
        }

        let mut rest = [0_u8; 2];
        let mut descriptor = libc::pollfd {
            fd: libc::STDIN_FILENO,
            events: libc::POLLIN,
            revents: 0,
        };
        if unsafe { libc::poll(&mut descriptor, 1, 30) } <= 0 {
            return Ok(Key::Escape);
        }
        let count = unsafe { libc::read(libc::STDIN_FILENO, rest.as_mut_ptr().cast(), 2) };
        if count == 2 && rest[0] == b'[' {
            return Ok(match rest[1] {
                b'A' => Key::Up,
                b'B' => Key::Down,
                _ => Key::Escape,
            });
        }
        Ok(Key::Escape)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &self.original) };
        print!("\x1b[?25h\x1b[?1049l");
        let _ = io::stdout().flush();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Up,
    Down,
    Escape,
}
