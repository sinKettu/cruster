use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    io::{self, BufRead, BufReader, Write},
};

use clap::ArgMatches;

use crate::{
    audit::RuleResult, cli::CrusterCLIError, http_storage::serializable::SerializableProxyData,
};

fn read_request_urls(reader: impl BufRead) -> io::Result<HashMap<usize, String>> {
    let mut urls = HashMap::new();
    for line in reader.lines() {
        let pair: SerializableProxyData = serde_json::from_str(&line?)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        urls.insert(pair.index, pair.request.get_uri());
    }
    Ok(urls)
}

fn request_line_with_url(finding: &RuleResult, urls: &HashMap<usize, String>) -> String {
    let first_line = finding.get_initial_request_first_line().trim_end();
    match urls.get(&finding.get_pair_index()) {
        Some(url) => {
            let mut parts = first_line.split_whitespace();
            let method = parts.next().unwrap_or_default();
            let _target = parts.next();
            let version = parts.next().unwrap_or_default();
            format!("{} {} {}", method, url, version)
        }
        None => first_line.to_string(),
    }
}

pub(crate) struct AuditPrintConfig {
    pub(crate) audit_name: String,
    pub(crate) all: bool,
    pub(crate) index: usize,
    pub(crate) wout_body: bool,
    pub(crate) no_data: bool,
    pub(crate) init_data: bool,
    pub(crate) sort: bool,
    pub(crate) sort_severity: bool,
    pub(crate) sort_ruleid: bool,
}

impl TryFrom<&ArgMatches> for AuditPrintConfig {
    type Error = CrusterCLIError;
    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let audit_name = value.get_one::<String>("name").unwrap().to_owned();

        if value.get_flag("all") {
            return Ok(AuditPrintConfig {
                audit_name,
                all: true,
                index: 0,
                wout_body: false,
                no_data: false,
                init_data: false,
                sort: false,
                sort_ruleid: false,
                sort_severity: false,
            });
        } else {
            let index = value.get_one::<usize>("index").unwrap().to_owned();
            let wout_body = value.get_flag("without-body");
            let init_data = value.get_flag("initial-data");
            let no_data = value.get_flag("no-data");
            let sort_severity = value.get_flag("sort-severity");
            let sort_ruleid = value.get_flag("sort-ruleid");
            let sort = sort_severity || sort_ruleid;

            return Ok(AuditPrintConfig {
                audit_name,
                all: false,
                index,
                wout_body,
                no_data,
                init_data,
                sort,
                sort_severity,
                sort_ruleid,
            });
        }
    }
}

fn print_http_message<T: Display>(
    output: &mut impl Write,
    shift: &str,
    label: T,
    data: &str,
    wout_body: bool,
) -> io::Result<()> {
    writeln!(output, "{}{}\n", shift, label)?;

    let splitted_request: Vec<&str> = data.split("\n").collect();
    for line in splitted_request {
        if line == "\r" && wout_body {
            writeln!(output, "{}\t >", shift)?;
            break;
        }

        writeln!(output, "{}\t > {}", shift, line)?;
    }

    writeln!(output)
}

fn print_single_result(
    output: &mut impl Write,
    print_conf: &AuditPrintConfig,
    line: String,
    urls: &HashMap<usize, String>,
) -> io::Result<bool> {
    let finding = serde_json::from_str::<RuleResult>(&line)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    if finding.get_id() != print_conf.index {
        return Ok(false);
    }

    writeln!(output, "{:<10}  {:<}", "Rule ID:", finding.get_rule_id())?;
    writeln!(output, "{:<10}  {:<}", "Pair ID:", finding.get_pair_index())?;
    if let Some(url) = urls.get(&finding.get_pair_index()) {
        writeln!(output, "{:<10}  {}", "URL:", url)?;
    }
    writeln!(output, "{:<10}  {:<}", "Severity:", finding.get_severity())?;
    writeln!(output, "{:<10}  {:<}", "Protocol:", finding.get_protocol())?;
    writeln!(output, "{:<10}  {:<}", "Type:", finding.get_type())?;
    writeln!(output, "{:<10}  {:<}", "About:", finding.get_about())?;

    let actual_findings = finding.get_findings();
    writeln!(output, "\nFindings:")?;
    for (finding_name, (extracted, send_results)) in actual_findings.iter() {
        let joined_extracted_items = extracted.join(", ");

        writeln!(output, "\t{:<10}:  {:<}", "Name", finding_name)?;
        writeln!(
            output,
            "\t{:<10}:  {:<}",
            "Extracted", joined_extracted_items
        )?;
        writeln!(output)?;

        if !print_conf.no_data {
            for send_result in send_results {
                let str_payloads = send_result.payloads.join(" || ");

                print_http_message(
                    output,
                    "\t",
                    format!("Request (payload='{}'):", &str_payloads),
                    &send_result.request,
                    print_conf.wout_body,
                )?;

                print_http_message(
                    output,
                    "\t",
                    format!("Response (payload='{}'):", &str_payloads),
                    &send_result.response,
                    print_conf.wout_body,
                )?;
            }

            if print_conf.init_data {
                print_http_message(
                    output,
                    "",
                    "Initial request:",
                    finding.get_initial_request(),
                    print_conf.wout_body,
                )?;

                print_http_message(
                    output,
                    "",
                    "Initial response:",
                    finding.get_initial_response(),
                    print_conf.wout_body,
                )?;
            }
        }
    }

    Ok(true)
}

fn print_finding_briefly(
    output: &mut impl Write,
    finding: &RuleResult,
    urls: &HashMap<usize, String>,
) -> io::Result<()> {
    let all_findings = finding.get_all_findings_as_str();
    let all_findings_cutted = if all_findings.len() > 69 {
        &all_findings[..69]
    } else {
        &all_findings
    };

    writeln!(
        output,
        "{:>4}  {:>4}  {:<8} {:<30}  {:<70}  {:<}",
        finding.get_id(),
        finding.get_pair_index(),
        finding.get_severity(),
        finding.get_rule_id(),
        all_findings_cutted,
        request_line_with_url(finding, urls)
    )
}

fn print_all(
    output: &mut impl Write,
    print_conf: &AuditPrintConfig,
    reader: impl BufRead,
    urls: &HashMap<usize, String>,
) -> io::Result<()> {
    let mut lines: Vec<RuleResult> = vec![];
    for possible_line in reader.lines() {
        match possible_line {
            Ok(line) => {
                let finding = serde_json::from_str::<RuleResult>(&line)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

                if print_conf.sort {
                    lines.push(finding);
                    continue;
                } else {
                    print_finding_briefly(output, &finding, urls)?;
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    if !lines.is_empty() {
        if print_conf.sort_ruleid {
            lines.sort_by(|a, b| a.get_rule_id().cmp(b.get_rule_id()));
        }

        if print_conf.sort_severity {
            lines.sort_by(|a, b| a.get_num_severity().cmp(&b.get_num_severity()));
        }

        for line in lines.iter() {
            print_finding_briefly(output, line, urls)?;
        }
    }

    Ok(())
}

fn print_results(
    output: &mut impl Write,
    print_conf: &AuditPrintConfig,
    reader: impl BufRead,
    urls: &HashMap<usize, String>,
) -> io::Result<()> {
    if print_conf.all {
        print_all(output, print_conf, reader, urls)?;
    } else {
        for possible_line in reader.lines() {
            match possible_line {
                Ok(line) => {
                    if print_single_result(output, print_conf, line, urls)? {
                        return Ok(());
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        writeln!(
            output,
            "Cannot get finding with index {} in results of audit '{}'",
            print_conf.index, &print_conf.audit_name
        )?;
    }

    Ok(())
}

fn write_results(
    output: &mut impl Write,
    print_conf: &AuditPrintConfig,
    reader: impl BufRead,
    urls: &HashMap<usize, String>,
) -> Result<(), CrusterCLIError> {
    // A consumer such as `head` may close stdout before all results are written.
    // Stop immediately in that case, but preserve all other errors.
    match print_results(output, print_conf, reader, urls).and_then(|()| output.flush()) {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result.map_err(CrusterCLIError::from),
    }
}

pub(crate) async fn exec(
    print_conf: AuditPrintConfig,
    results: String,
    http_data_path: &str,
) -> Result<(), CrusterCLIError> {
    let fin = fs::OpenOptions::new().read(true).open(&results)?;
    let reader = BufReader::new(fin);
    let urls = match fs::File::open(http_data_path) {
        Ok(file) => read_request_urls(BufReader::new(file))?,
        Err(err) if err.kind() == io::ErrorKind::NotFound => HashMap::new(),
        Err(err) => return Err(err.into()),
    };
    let stdout = io::stdout();
    write_results(&mut stdout.lock(), &print_conf, reader, &urls)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FINDING: &str = r#"{"id":0,"rule_id":"test-rule","about":"Test finding","protocol":"http","type":"passive","pair_index":0,"severity":"info","findings":{"test":[["value"],[]]},"initial_request":"GET / HTTP/1.1\r\n\r\n","initial_response":"HTTP/1.1 200 OK\r\n\r\n"}"#;

    fn config(all: bool, sort: bool) -> AuditPrintConfig {
        AuditPrintConfig {
            audit_name: "test".into(),
            all,
            index: 0,
            wout_body: false,
            no_data: false,
            init_data: true,
            sort,
            sort_severity: sort,
            sort_ruleid: false,
        }
    }

    struct FailingWriter {
        kind: io::ErrorKind,
        fail_on_flush: bool,
    }

    impl Write for FailingWriter {
        fn write(&mut self, data: &[u8]) -> io::Result<usize> {
            if self.fail_on_flush {
                Ok(data.len())
            } else {
                Err(io::Error::from(self.kind))
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(self.kind))
        }
    }

    #[test]
    fn closed_pipe_is_success_for_all_output_modes() {
        for (all, sort) in [(true, false), (true, true), (false, false)] {
            for fail_on_flush in [false, true] {
                let mut output = FailingWriter {
                    kind: io::ErrorKind::BrokenPipe,
                    fail_on_flush,
                };
                // Invalid trailing input must not be read after a failed write.
                let input = if !sort && !fail_on_flush {
                    format!("{}\ninvalid json\n", FINDING)
                } else {
                    format!("{}\n", FINDING)
                };
                assert!(write_results(
                    &mut output,
                    &config(all, sort),
                    input.as_bytes(),
                    &HashMap::new()
                )
                .is_ok());
            }
        }
    }

    #[test]
    fn other_write_and_flush_errors_are_reported() {
        for fail_on_flush in [false, true] {
            let mut output = FailingWriter {
                kind: io::ErrorKind::PermissionDenied,
                fail_on_flush,
            };
            assert!(write_results(
                &mut output,
                &config(true, false),
                FINDING.as_bytes(),
                &HashMap::new()
            )
            .is_err());
        }
    }

    #[test]
    fn malformed_results_are_reported() {
        assert!(write_results(
            &mut Vec::new(),
            &config(true, false),
            &b"invalid json\n"[..],
            &HashMap::new()
        )
        .is_err());
    }

    #[test]
    fn normal_output_is_preserved() {
        for (all, sort) in [(true, false), (true, true), (false, false)] {
            let mut output = Vec::new();
            write_results(
                &mut output,
                &config(all, sort),
                FINDING.as_bytes(),
                &HashMap::new(),
            )
            .unwrap();
            let output = String::from_utf8(output).unwrap();
            assert!(output.contains("test-rule"));
            assert!(output.contains("GET / HTTP/1.1"));
            assert!(output.ends_with('\n'));
        }
    }

    #[test]
    fn pair_id_and_full_url_are_printed_in_all_modes() {
        let mut finding: serde_json::Value = serde_json::from_str(FINDING).unwrap();
        finding["pair_index"] = 42.into();
        finding["initial_request"] = "GET /search?q=test HTTP/1.1\r\n\r\n".into();
        let input = serde_json::to_string(&finding).unwrap();
        let urls = HashMap::from([(42, "https://example.com:8443/search?q=test".into())]);

        for (all, sort) in [(true, false), (true, true), (false, false)] {
            let mut output = Vec::new();
            let mut conf = config(all, sort);
            conf.no_data = true;
            write_results(&mut output, &conf, input.as_bytes(), &urls).unwrap();
            let output = String::from_utf8(output).unwrap();
            assert!(output.contains("https://example.com:8443/search?q=test"));
            if all {
                let columns: Vec<_> = output.split_whitespace().collect();
                assert_eq!(&columns[..2], &["0", "42"]);
                assert!(output.contains("GET https://example.com:8443/search?q=test HTTP/1.1"));
            } else {
                assert!(output.contains("Pair ID:    42"));
            }
        }
    }

    #[test]
    fn urls_are_loaded_by_pair_id_with_scheme_port_and_query() {
        let input = r#"{"index":42,"request":{"method":"GET","scheme":"https://","host":"example.com:8443","path":"/search","query":"?q=test","version":"HTTP/1.1","headers":[],"body":null},"response":null}"#;
        let urls = read_request_urls(input.as_bytes()).unwrap();
        assert_eq!(
            urls.get(&42).unwrap(),
            "https://example.com:8443/search?q=test"
        );
    }
}
