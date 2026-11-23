pub(super) fn is_cookie_header(name: &str) -> bool {
    name.eq_ignore_ascii_case("cookie") || name.eq_ignore_ascii_case("set-cookie")
}

/// Redact only HTTP headers, preserving the body and line endings verbatim.
pub(super) fn strip_cookies(message: &str) -> String {
    let mut output = String::with_capacity(message.len());
    let mut in_headers = true;
    let mut cookie_continuation = false;

    for (position, line) in message.split_inclusive('\n').enumerate() {
        if position == 0 || !in_headers {
            output.push_str(line);
            continue;
        }
        let content = line.trim_end_matches('\n').trim_end_matches('\r');
        if content.is_empty() {
            in_headers = false;
            output.push_str(line);
            continue;
        }
        if content.starts_with([' ', '\t']) {
            if !cookie_continuation {
                output.push_str(line);
            }
            continue;
        }
        cookie_continuation = false;
        if let Some((name, _)) = content.split_once(':') {
            if is_cookie_header(name.trim()) {
                output.push_str(name);
                output.push_str(": stripped");
                output.push_str(&line[content.len()..]);
                cookie_continuation = true;
                continue;
            }
        }
        output.push_str(line);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_mixed_case_and_repeated_cookie_headers_but_not_body() {
        for newline in ["\n", "\r\n"] {
            let input = [
                "HTTP/1.1 200 OK", "Cookie: secret=1", "sEt-CoOkIe: token=2; HttpOnly",
                "Set-Cookie: other=3", "X-Cookie: visible", "", "Cookie: body is unchanged",
            ].join(newline);
            let expected = [
                "HTTP/1.1 200 OK", "Cookie: stripped", "sEt-CoOkIe: stripped",
                "Set-Cookie: stripped", "X-Cookie: visible", "", "Cookie: body is unchanged",
            ].join(newline);
            assert_eq!(strip_cookies(&input), expected);
        }
    }

    #[test]
    fn strips_folded_cookie_values_and_unterminated_headers() {
        let input = "GET / HTTP/1.1\r\nCookie: first=secret;\r\n second=secret\r\n\tthird=secret\r\nHost: example.com\r\nSet-Cookie: last=secret";
        assert_eq!(strip_cookies(input), "GET / HTTP/1.1\r\nCookie: stripped\r\nHost: example.com\r\nSet-Cookie: stripped");
        let unchanged = "GET / HTTP/1.1\nX-Test: value\n continuation\n\nbody\n";
        assert_eq!(strip_cookies(unchanged), unchanged);
        assert_eq!(strip_cookies(""), "");
    }
}
