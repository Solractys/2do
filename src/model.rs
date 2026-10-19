use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    #[serde(skip)]
    pub selected: usize,
}

impl TaskList {
    pub fn add(&mut self, title: String) {
        self.tasks.push(Task { title, done: false });
        self.selected = self.tasks.len() - 1;
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.tasks.is_empty() {
            return;
        }
        self.selected = self
            .selected
            .saturating_add_signed(delta)
            .min(self.tasks.len() - 1);
    }

    pub fn move_task(&mut self, delta: isize) {
        if self.tasks.is_empty() {
            return;
        }
        let target = self
            .selected
            .saturating_add_signed(delta)
            .min(self.tasks.len() - 1);
        self.tasks.swap(self.selected, target);
        self.selected = target;
    }

    pub fn remove_selected(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        self.tasks.remove(self.selected);
        self.selected = self.selected.min(self.tasks.len().saturating_sub(1));
    }

    pub fn toggle_selected(&mut self) {
        if let Some(task) = self.tasks.get_mut(self.selected) {
            task.done = !task.done;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_task_changes_priority_and_selection() {
        let mut list = TaskList::default();
        list.add("primeira".into());
        list.add("segunda".into());
        list.move_task(-1);
        assert_eq!(list.tasks[0].title, "segunda");
        assert_eq!(list.selected, 0);
    }

    #[test]
    fn removing_last_task_keeps_valid_selection() {
        let mut list = TaskList::default();
        list.add("única".into());
        list.remove_selected();
        assert_eq!(list.selected, 0);
        assert!(list.tasks.is_empty());
    }
}
