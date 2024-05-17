pub mod entry;
mod model_io;

use crate::entry::EntryGroup;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Empty,
    Loaded,
    Done,
}

#[derive(Debug, Default)]
pub struct Model {
    pub file: String,
    pub(crate) entries: Vec<EntryGroup>,
    pub running_state: RunningState,
    pub idx_entrygroup: usize,
}

impl Model {
    pub fn new(file: String) -> Model {
        Model {
            file,
            entries: vec![],
            running_state: RunningState::Empty,
            idx_entrygroup: 0,
        }
    }

    pub fn next_entrygroup(&mut self) {
        self.idx_entrygroup = (self.idx_entrygroup + 1) % self.entries.len();
    }

    pub fn previous_entrygroup(&mut self) {
        self.idx_entrygroup = match self.idx_entrygroup {
            0 => self.entries.len() - 1,
            _ => self.idx_entrygroup - 1,
        };
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::entry::Entry;

    pub fn make_test_entry_group() -> EntryGroup {
        let entry1 = Entry {
            command: String::from("command1"),
            short_info: String::from("Short description 1"),
            long_info: String::from("Long description 1"),
        };

        let entry2 = Entry {
            command: String::from("command2"),
            short_info: String::from("Short description 2"),
            long_info: String::from("Long description 2"),
        };

        let description = String::from("description");

        EntryGroup::new(description, vec![entry1, entry2])
    }

    #[test]
    fn create_default_model() {
        let model = Model::default();
        assert_eq!(model.file.len(), 0);
        assert!(model.entries.is_empty());
        assert_eq!(model.running_state, RunningState::Empty);
    }

    #[test]
    fn create_model() {
        let file = String::from("test.cache");
        let entrygroup = make_test_entry_group();

        let mut model = Model::new(file);
        model.entries = vec![entrygroup];
        assert_eq!(model.file.len(), 10);
        assert_eq!(model.entries.len(), 1);
        assert_eq!(model.entries[0].entries.len(), 2);
        assert_eq!(model.running_state, RunningState::Empty);
        assert_eq!(model.entries[0].entries[0].command, "command1");
    }

    #[test]
    fn increase_index_entry() {
        let mut model = Model::default();

        let mut egs: Vec<EntryGroup> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };
            let eg = EntryGroup::new("".to_owned(), vec![entry]);

            egs.push(eg);
        }

        model.entries = egs;

        assert_eq!(model.idx_entrygroup, 0);
        model.next_entrygroup();
        assert_eq!(model.idx_entrygroup, 1);
        model.next_entrygroup();
        assert_eq!(model.idx_entrygroup, 2);
        model.next_entrygroup();
        model.next_entrygroup();
        model.next_entrygroup();
        model.next_entrygroup();
        model.next_entrygroup();
        model.next_entrygroup();
        model.next_entrygroup();
        assert_eq!(model.idx_entrygroup, 9);
        model.next_entrygroup();
        assert_eq!(model.idx_entrygroup, 0);
    }

    #[test]
    fn decrease_index_entry() {
        let mut model = Model::default();

        let mut egs: Vec<EntryGroup> = vec![];
        for _ in 0..10 {
            let entry = Entry {
                command: "".to_owned(),
                short_info: "".to_owned(),
                long_info: "".to_owned(),
            };
            let eg = EntryGroup::new("".to_owned(), vec![entry]);

            egs.push(eg);
        }

        model.entries = egs;

        assert_eq!(model.idx_entrygroup, 0);
        model.previous_entrygroup();
        assert_eq!(model.idx_entrygroup, 9);
        model.previous_entrygroup();
        assert_eq!(model.idx_entrygroup, 8);
        model.previous_entrygroup();
        model.previous_entrygroup();
        model.previous_entrygroup();
        model.previous_entrygroup();
        model.previous_entrygroup();
        model.previous_entrygroup();
        model.previous_entrygroup();
        assert_eq!(model.idx_entrygroup, 1);
        model.previous_entrygroup();
        assert_eq!(model.idx_entrygroup, 0);
    }
}
