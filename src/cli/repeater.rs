use std::fs::File;
use serde_json as json;
use std::io::{BufReader, BufRead};
use crate::siv_ui::repeater::{RepeaterState, RepeaterStateSerializable};

pub(crate) mod list;
pub(crate) mod show;


pub(crate) struct RepeaterIterator {
    reader: BufReader<File>
}

impl RepeaterIterator {
    pub(crate) fn new(path: &str) -> RepeaterIterator {
        let file = File::open(path).unwrap();
        let iter = RepeaterIterator {
            reader: BufReader::new(file)
        };

        return iter;
    }
}

impl Iterator for RepeaterIterator {
    type Item = RepeaterState;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::default();
        let len = self.reader.read_line(&mut buf).unwrap();
        if len == 0 {
            return None
        }

        let ser_repeater = json::from_str::<RepeaterStateSerializable>(&buf).unwrap();
        let repeater = RepeaterState::try_from(ser_repeater).unwrap();

        return Some(repeater);
    }
}
