use crate::model::TaskList;
use std::io::{self, Write};

#[derive(Copy, Clone)]
pub enum Screen<'a> {
    List,
    Input { label: &'a str, value: &'a str },
    Help,
}

pub fn draw(
    list: &TaskList,
    screen: Screen<'_>,
    terminal_width: usize,
    terminal_height: usize,
    message: &str,
) -> io::Result<()> {
    let width = terminal_width.saturating_sub(2).clamp(30, 76);
    let inner = width - 2;
    let body_height = terminal_height.saturating_sub(9).clamp(8, 14);
    let total_height = body_height + 7;
    let top = terminal_height.saturating_sub(total_height) / 2;
    let left = terminal_width.saturating_sub(width) / 2;
    let indent = " ".repeat(left);
    let mut out = format!("\x1b[2J\x1b[H{}", "\r\n".repeat(top));

    border(&mut out, &indent, '╭', '─', '╮', inner);
    row(
        &mut out,
        &indent,
        inner,
        &format!(
            "\x1b[1;36m2do\x1b[0m  \x1b[2m{} tarefa(s)\x1b[0m",
            list.tasks.len()
        ),
        15 + digits(list.tasks.len()),
    );
    border(&mut out, &indent, '├', '─', '┤', inner);

    match screen {
        Screen::List => draw_list(&mut out, &indent, inner, body_height, list),
        Screen::Input { label, value } => {
            draw_input(&mut out, &indent, inner, body_height, label, value)
        }
        Screen::Help => draw_help(&mut out, &indent, inner, body_height),
    }

    border(&mut out, &indent, '├', '─', '┤', inner);
    let footer = match screen {
        Screen::List => "j/k mover  •  a criar  •  r renomear  •  J/K prioridade",
        Screen::Input { .. } => "Enter salvar  •  Esc cancelar",
        Screen::Help => "? ou Esc para voltar",
    };
    centered_row(&mut out, &indent, inner, footer, "\x1b[2m");
    let status = if message.is_empty() {
        if matches!(screen, Screen::List) {
            "espaço concluir  •  dd apagar  •  ? ajuda  •  q sair"
        } else {
            ""
        }
    } else {
        message
    };
    centered_row(
        &mut out,
        &indent,
        inner,
        status,
        if message.is_empty() {
            "\x1b[2m"
        } else {
            "\x1b[33m"
        },
    );
    border(&mut out, &indent, '╰', '─', '╯', inner);
    print!("{out}");
    io::stdout().flush()
}

fn draw_list(out: &mut String, indent: &str, inner: usize, height: usize, list: &TaskList) {
    if list.tasks.is_empty() {
        blank_rows(out, indent, inner, height / 2);
        centered_row(out, indent, inner, "Sua lista está vazia.", "\x1b[2m");
        centered_row(
            out,
            indent,
            inner,
            "Pressione a para criar uma tarefa.",
            "\x1b[36m",
        );
        blank_rows(out, indent, inner, height.saturating_sub(height / 2 + 2));
        return;
    }
    let start = list.selected.saturating_sub(height.saturating_sub(1));
    let shown: Vec<_> = list
        .tasks
        .iter()
        .enumerate()
        .skip(start)
        .take(height)
        .collect();
    for (index, task) in &shown {
        let selected = *index == list.selected;
        let cursor = if selected { "❯" } else { " " };
        let mark = if task.done { "✓" } else { "○" };
        let title = truncate(&task.title, inner.saturating_sub(10));
        let style = match (selected, task.done) {
            (true, true) => "\x1b[48;5;236m\x1b[2m",
            (true, false) => "\x1b[48;5;236m\x1b[1m",
            (false, true) => "\x1b[2m",
            _ => "",
        };
        let content = format!(
            "{style} {cursor} \x1b[36m{mark}\x1b[0m{style} {:>2}. {title}\x1b[0m",
            index + 1
        );
        row(
            out,
            indent,
            inner,
            &content,
            8 + digits(index + 1) + title.chars().count(),
        );
    }
    blank_rows(out, indent, inner, height.saturating_sub(shown.len()));
}

fn draw_input(
    out: &mut String,
    indent: &str,
    inner: usize,
    height: usize,
    label: &str,
    value: &str,
) {
    let card_width = inner.saturating_sub(8).clamp(20, 50);
    let mut lines = wrap(value, card_width - 4);
    if lines.is_empty() {
        lines.push(String::new());
    }
    if let Some(last) = lines.last_mut() {
        last.push('_');
    }
    let lines: Vec<_> = lines
        .into_iter()
        .rev()
        .take(5)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    let card_height = lines.len() + 3;
    let top = height.saturating_sub(card_height) / 2;
    blank_rows(out, indent, inner, top);
    centered_row(out, indent, inner, label, "\x1b[1;36m");
    small_border(out, indent, inner, card_width, '╭', '╮');
    for line in &lines {
        small_row(out, indent, inner, card_width, line);
    }
    small_border(out, indent, inner, card_width, '╰', '╯');
    blank_rows(out, indent, inner, height.saturating_sub(top + card_height));
}

fn draw_help(out: &mut String, indent: &str, inner: usize, height: usize) {
    let shortcuts = [
        ("j / k", "navegar"),
        ("a", "criar tarefa"),
        ("r", "renomear"),
        ("J / K", "mudar prioridade"),
        ("dd", "apagar"),
        ("espaço", "concluir"),
    ];
    let top = height.saturating_sub(shortcuts.len() + 1) / 2;
    blank_rows(out, indent, inner, top);
    centered_row(out, indent, inner, "ATALHOS", "\x1b[1m");
    for (key, action) in shortcuts {
        centered_row(
            out,
            indent,
            inner,
            &format!("{key:<8} {action}"),
            "\x1b[36m",
        );
    }
    blank_rows(
        out,
        indent,
        inner,
        height.saturating_sub(top + shortcuts.len() + 1),
    );
}

fn row(out: &mut String, indent: &str, inner: usize, content: &str, visible: usize) {
    out.push_str(&format!(
        "{indent}\x1b[36m│\x1b[0m {content}{} \x1b[36m│\x1b[0m\r\n",
        " ".repeat(inner.saturating_sub(visible + 2))
    ));
}

fn centered_row(out: &mut String, indent: &str, inner: usize, text: &str, style: &str) {
    let length = text.chars().count();
    let left = inner.saturating_sub(length) / 2;
    let right = inner.saturating_sub(length + left);
    out.push_str(&format!(
        "{indent}\x1b[36m│\x1b[0m{}{style}{text}\x1b[0m{}\x1b[36m│\x1b[0m\r\n",
        " ".repeat(left),
        " ".repeat(right)
    ));
}

fn blank_rows(out: &mut String, indent: &str, inner: usize, count: usize) {
    for _ in 0..count {
        row(out, indent, inner, "", 0);
    }
}

fn border(out: &mut String, indent: &str, left: char, fill: char, right: char, inner: usize) {
    out.push_str(&format!(
        "{indent}\x1b[36m{left}{}{right}\x1b[0m\r\n",
        fill.to_string().repeat(inner)
    ));
}

fn small_border(
    out: &mut String,
    indent: &str,
    outer: usize,
    width: usize,
    left: char,
    right: char,
) {
    let pad = outer.saturating_sub(width) / 2;
    let content = format!(
        "{}\x1b[2m{left}{}{right}\x1b[0m",
        " ".repeat(pad.saturating_sub(1)),
        "─".repeat(width - 2)
    );
    row(out, indent, outer, &content, pad.saturating_sub(1) + width);
}

fn small_row(out: &mut String, indent: &str, outer: usize, width: usize, text: &str) {
    let pad = outer.saturating_sub(width) / 2;
    let content = format!(
        "{}\x1b[2m│\x1b[0m {text}{} \x1b[2m│\x1b[0m",
        " ".repeat(pad.saturating_sub(1)),
        " ".repeat(width.saturating_sub(text.chars().count() + 4))
    );
    row(out, indent, outer, &content, pad.saturating_sub(1) + width);
}

fn wrap(value: &str, width: usize) -> Vec<String> {
    let chars: Vec<_> = value.chars().collect();
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut start = 0;
    while start < chars.len() {
        let limit = (start + width).min(chars.len());
        let end =
            if limit == chars.len() || chars.get(limit).is_some_and(|char| char.is_whitespace()) {
                limit
            } else {
                chars[start..limit]
                    .iter()
                    .rposition(|char| char.is_whitespace())
                    .map(|position| start + position)
                    .filter(|position| *position > start)
                    .unwrap_or(limit)
            };
        lines.push(chars[start..end].iter().collect::<String>());
        start = end;
        while chars.get(start).is_some_and(|char| char.is_whitespace()) {
            start += 1;
        }
    }
    lines
}

fn truncate(value: &str, max: usize) -> String {
    let chars: Vec<_> = value.chars().collect();
    if chars.len() <= max {
        value.to_string()
    } else {
        format!(
            "{}…",
            chars[..max.saturating_sub(1)].iter().collect::<String>()
        )
    }
}

fn digits(value: usize) -> usize {
    value.max(1).ilog10() as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_long_input_into_compact_lines() {
        assert_eq!(wrap("abcdefghij", 4), ["abcd", "efgh", "ij"]);
        assert_eq!(
            wrap("uma tarefa bem longa", 10),
            ["uma tarefa", "bem longa"]
        );
    }
}
