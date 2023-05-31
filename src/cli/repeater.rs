use std::fs::File;
use serde_json as json;
use super::CrusterCLIError;
use std::io::{BufReader, BufRead, Write};
use crate::siv_ui::repeater::{RepeaterState, RepeaterStateSerializable};

pub(crate) mod list;
pub(crate) mod show;
pub(crate) mod exec;
pub(crate) mod edit;
pub(crate) mod add;


pub(crate) struct RepeaterIterator {
    reader: BufReader<File>
}

impl RepeaterIterator {
    pub(crate) fn new(path: &str) -> RepeaterIterator {
        let file = File::open(path).expect("No file with saved repeaters exists!");
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
        while buf.trim().len() == 0 {
            let len = self.reader.read_line(&mut buf).unwrap();
            if len == 0 {
                return None
            }
        }

        let ser_repeater = json::from_str::<RepeaterStateSerializable>(&buf).unwrap();
        let repeater = RepeaterState::try_from(ser_repeater).unwrap();

        return Some(repeater);
    }
}

pub(crate) fn update_repeaters(path: &str, repeater: &RepeaterState, number: usize) -> Result<(), CrusterCLIError> {
    let buf = if std::path::Path::new(path).is_file() {
        let file = File::open(path)?;
        let fin = BufReader::new(file);
        let mut buf: Vec<Option<String>> = Vec::with_capacity(20);

        for (i, line) in fin.lines().enumerate() {
            if let Ok(repeater_str) = line {
                if i != number {
                    buf.push(Some(repeater_str));
                }
                else {
                    buf.push(None);
                }
            }
        }

        buf
    }
    else {
        vec![None]
    };

    let mut found_flag: bool = false;
    let mut fout = std::fs::OpenOptions::new().create(true).write(true).open(path)?;
    for possible_repeater_str in buf {
        if let Some(repeater_str) = possible_repeater_str {
            let line = format!("{}\n", repeater_str);
            let _ = fout.write(line.as_bytes())?;
        }
        else {
            let repeater_ser = RepeaterStateSerializable::from(repeater);
            let repeater_str = json::to_string(&repeater_ser)?;
            let line = format!("{}\n", repeater_str);
            let _ = fout.write(line.as_bytes())?;
            found_flag = true;
        }
    }

    if !found_flag {
        let repeater_ser = RepeaterStateSerializable::from(repeater);
        let repeater_str = json::to_string(&repeater_ser)?;
        let line = format!("{}\n", repeater_str);
        let _ = fout.write(line.as_bytes())?;
    }

    Ok(())
}

pub(crate) fn trim_body(req_or_res: &str) -> String {
    let mut result_wout_body = String::with_capacity(req_or_res.len());
    for line in req_or_res.split("\n") {
        if line.trim().is_empty() {
            result_wout_body.push_str(line);
            result_wout_body.push_str("\n");
            break
        }

        result_wout_body.push_str(line);
        result_wout_body.push_str("\n");
    }

    return result_wout_body;
}
