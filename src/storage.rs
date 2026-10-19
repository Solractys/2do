use crate::model::TaskList;
use std::{env, fs, io, path::PathBuf};

pub fn data_path() -> PathBuf {
    if let Some(path) = env::var_os("TODO_TUI_DATA") {
        return path.into();
    }
    let base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("2do/tasks.json")
}

pub fn load() -> io::Result<TaskList> {
    let path = data_path();
    if !path.exists() {
        return Ok(TaskList::default());
    }
    let contents = fs::read_to_string(path)?;
    serde_json::from_str(&contents)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn save(list: &TaskList) -> io::Result<()> {
    let path = data_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(list).map_err(io::Error::other)?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, json)?;
    fs::rename(temporary, path)
}
