use super::CrusterCLIError;
use crate::siv_ui::repeater::{RepeaterState, RepeaterStateSerializable};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub(crate) mod add;
pub(crate) mod edit;
pub(crate) mod exec;
pub(crate) mod list;
pub(crate) mod operations;
pub(crate) mod parsing;
pub(crate) mod show;

pub(crate) struct RepeaterIterator {
    reader: std::io::Lines<BufReader<File>>,
    line: usize,
}

impl RepeaterIterator {
    pub(crate) fn new(path: &str) -> Result<Self, CrusterCLIError> {
        Ok(Self {
            reader: BufReader::new(File::open(path)?).lines(),
            line: 0,
        })
    }
}

impl Iterator for RepeaterIterator {
    type Item = Result<RepeaterState, CrusterCLIError>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = self.reader.next()?;
            self.line += 1;
            match line {
                Ok(line) if line.trim().is_empty() => continue,
                Ok(line) => {
                    return Some(
                        decode_record(&line, self.line)
                            .and_then(|r| RepeaterState::try_from(r).map_err(Into::into)),
                    )
                }
                Err(error) => return Some(Err(error.into())),
            }
        }
    }
}

fn decode_record(line: &str, number: usize) -> Result<RepeaterStateSerializable, CrusterCLIError> {
    serde_json::from_str(line)
        .map_err(|e| format!("Cannot parse repeater line {}: {}", number, e).into())
}

pub(crate) fn read_repeaters(
    path: &Path,
) -> Result<Vec<RepeaterStateSerializable>, CrusterCLIError> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };
    let mut records = Vec::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if !line.trim().is_empty() {
            records.push(decode_record(&line, i + 1)?);
        }
    }
    Ok(records)
}

/// The one persistence algorithm used by CLI and MCP. Never truncate the live file.
pub(crate) fn write_repeaters(
    path: &Path,
    records: &[RepeaterStateSerializable],
) -> Result<(), CrusterCLIError> {
    let temp = path.with_file_name(format!(".repeater-{:016x}.tmp", rand::random::<u64>()));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        for record in records {
            serde_json::to_writer(&mut file, record)?;
            file.write_all(b"\n")?;
        }
        file.sync_all()?;
        fs::rename(&temp, path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

pub(crate) fn update_repeaters(
    path: &Path,
    repeater: &RepeaterStateSerializable,
    number: usize,
) -> Result<(), CrusterCLIError> {
    let mut records = read_repeaters(path)?;
    let record = repeater.clone();
    if number < records.len() {
        records[number] = record;
    } else {
        records.push(record);
    }
    write_repeaters(path, &records)
}

pub(crate) fn trim_body(req_or_res: &str) -> String {
    let mut result_wout_body = String::with_capacity(req_or_res.len());
    for line in req_or_res.split("\n") {
        if line.trim().is_empty() {
            result_wout_body.push_str(line);
            result_wout_body.push_str("\n");
            break;
        }

        result_wout_body.push_str(line);
        result_wout_body.push_str("\n");
    }

    return result_wout_body;
}

fn format_output(message: &str, no_body: bool, no_cookie: bool) -> String {
    let message = if no_body { trim_body(message) } else { message.to_owned() };
    if no_cookie { super::output::strip_cookies(&message) } else { message }
}

#[cfg(test)]
mod output_tests {
    use super::format_output;

    #[test]
    fn no_cookie_combines_with_no_body_without_changing_source() {
        let message = "GET / HTTP/1.1\r\nCookie: session=secret\r\nHost: example.com\r\n\r\nbody secret";
        assert_eq!(format_output(message, false, false), message);
        let headers_only = format_output(message, true, true);
        assert!(headers_only.contains("Cookie: stripped\r\n"));
        assert!(!headers_only.contains("session=secret"));
        assert!(!headers_only.contains("body secret"));
        assert!(format_output(message, false, true).ends_with("body secret"));
        assert!(message.contains("Cookie: session=secret"));
    }
}
