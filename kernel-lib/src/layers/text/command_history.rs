use alloc::collections::VecDeque;
use alloc::string::String;
use core::cmp::min;

pub struct CommandHistory {
    commands: VecDeque<String>,
    select_idx: usize,
}


impl CommandHistory {
    #[inline]
    pub const fn new() -> Self {
        Self {
            commands: VecDeque::new(),
            select_idx: 0,
        }
    }


    #[inline]
    pub fn entry(&mut self, command_name: impl Into<String>) {
        self.commands
            .push_back(command_name.into());
        self.select_idx = self.commands.len();
    }


    pub fn history_up(&mut self) -> Option<&str> {
        self.select_idx = self
            .select_idx
            .saturating_sub(1);

        self.command_name()
    }


    pub fn history_down(&mut self) -> Option<&str> {
        let max_idx = self.commands.len() - 1;
        self.select_idx = min(max_idx, self.select_idx + 1);

        self.command_name()
    }


    fn command_name(&self) -> Option<&str> {
        self.commands
            .get(self.select_idx)
            .map(|str| str.as_str())
    }
}


#[cfg(test)]
mod tests {
    use crate::layers::text::command_history::CommandHistory;

    #[test]
    fn it_up_history() {
        let mut h = CommandHistory::new();
        h.entry("echo");

        assert_eq!(h.history_up(), Some("echo"))
    }


    #[test]
    fn it_two_times_up_history() {
        let mut h = CommandHistory::new();
        h.entry("echo");
        h.entry("sleep");

        assert_eq!(h.history_up(), Some("sleep"));
        assert_eq!(h.history_up(), Some("echo"));
        assert_eq!(h.history_up(), Some("echo"));
    }


    #[test]
    fn it_down_history() {
        let mut h = CommandHistory::new();
        h.entry("echo");

        assert_eq!(h.history_up(), Some("echo"));
        assert_eq!(h.history_up(), Some("echo"));
    }


    #[test]
    fn it_up_down_history() {
        let mut h = CommandHistory::new();
        h.entry("echo");
        h.entry("sleep");

        assert_eq!(h.history_up(), Some("sleep"));
        assert_eq!(h.history_up(), Some("echo"));
        assert_eq!(h.history_down(), Some("sleep"));
    }


    #[test]
    fn it_up_down_history2() {
        let mut h = CommandHistory::new();
        h.entry("echo");

        assert_eq!(Some("echo"), h.history_up());

        h.entry("sleep");

        assert_eq!(Some("sleep"), h.history_up());
        assert_eq!(Some("sleep"), h.history_down());
        assert_eq!(Some("echo"), h.history_up());

        h.entry("wakeup");

        assert_eq!(Some("wakeup"), h.history_down());
        assert_eq!(Some("sleep"), h.history_up());
    }
}
