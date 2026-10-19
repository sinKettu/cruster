use crate::http_storage;
use std::cmp::min;

pub(crate) mod show;
pub(super) mod follow;

pub(crate) fn print_briefly(pair: &http_storage::RequestResponsePair, with_header: bool) {
    let idx = pair.index;
    
    let (hostname, path, method) = if let Some(request) = pair.request.as_ref() {
        (request.get_hostname(), request.get_request_path(), request.method.clone())
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };

    let (status, length) = if let Some(response) = pair.response.as_ref() {
        let status = response.status.split(" ").next().unwrap().to_string();
        let length = response.body.len().to_string();
        (status, length)
    }
    else {
        ("<UNKNOWN>".to_string(), "<UNKNOWN>".to_string())
    };

    if with_header {
        println!("{:>6} {:>8} {:>32} {:>70} {:>11} {:>15}\n", "ID", "METHOD", "HOSTNAME", "PATH", "STATUS", "LENGTH");
    }

    println!(
        "{:>6} {:>8} {:>32} {:<70} {:>11} {:>15}",
        idx,
        &method[..min(8, method.len())],
        &hostname[..min(32, hostname.len())],
        &path[..min(70, path.len())],
        status,
        length
    );
}

pub(crate) fn print_urls(pair: &http_storage::RequestResponsePair) {
    if let Some(request) = pair.request.as_ref() {
        println!(
            "{:>6} {}",
            pair.index,
            request.uri
        )
    }
    else {
        println!(
            "{:>6} {}",
            pair.index,
            "<NONE>"
        )
    }
}

/// One storage line, with its original byte length for follow cursors.
pub(crate) struct HistoryLine {
    pub(crate) raw: String,
    pub(crate) byte_len: usize,
}

/// Read incrementally without losing a partial tail at EOF. Follow callers keep
/// `pending` between polls; snapshot callers may accept an unterminated last row.
pub(crate) fn read_history_line<R: std::io::BufRead>(
    reader: &mut R,
    pending: &mut Vec<u8>,
    complete_only: bool,
) -> std::io::Result<Option<HistoryLine>> {
    reader.read_until(b'\n', pending)?;
    if pending.is_empty() || (complete_only && pending.last() != Some(&b'\n')) {
        return Ok(None);
    }
    let byte_len = pending.len();
    let mut bytes = std::mem::take(pending);
    if bytes.last() == Some(&b'\n') {
        bytes.pop();
        if bytes.last() == Some(&b'\r') {
            bytes.pop();
        }
    }
    let raw = String::from_utf8(bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(Some(HistoryLine { raw, byte_len }))
}

pub(crate) fn parse_history_pair(raw: &str) -> Result<http_storage::RequestResponsePair, crate::cli::CrusterCLIError> {
    let record: http_storage::serializable::SerializableProxyData = serde_json::from_str(raw)?;
    Ok(record.try_into()?)
}

#[cfg(test)]
mod history_reader_tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn follow_retains_partial_utf8_until_newline() {
        let mut pending = Vec::new();
        let mut first = Cursor::new(b"old\r\n\xd0".to_vec());
        let old = read_history_line(&mut first, &mut pending, true).unwrap().unwrap();
        assert_eq!(old.raw, "old");
        assert_eq!(old.byte_len, 5);
        assert!(read_history_line(&mut first, &mut pending, true).unwrap().is_none());
        assert!(read_history_line(&mut first, &mut pending, true).unwrap().is_none());
        let mut appended = Cursor::new(b"\xb0\nnext\n".to_vec());
        let completed = read_history_line(&mut appended, &mut pending, true).unwrap().unwrap();
        assert_eq!(completed.raw, "а");
        assert_eq!(completed.byte_len, 3);
        assert_eq!(read_history_line(&mut appended, &mut pending, true).unwrap().unwrap().raw, "next");
        assert!(pending.is_empty());
    }

    #[test]
    fn snapshot_accepts_unterminated_tail_and_preserves_lone_cr() {
        let mut reader = Cursor::new(b"first\r\nlast\r");
        let mut pending = Vec::new();
        assert_eq!(read_history_line(&mut reader, &mut pending, false).unwrap().unwrap().raw, "first");
        assert_eq!(read_history_line(&mut reader, &mut pending, false).unwrap().unwrap().raw, "last\r");
        assert!(read_history_line(&mut reader, &mut pending, false).unwrap().is_none());
    }
}
