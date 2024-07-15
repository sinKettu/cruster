use std::{fmt::Display, fs, io::{BufRead, BufReader}};

use clap::ArgMatches;

use crate::{audit::RuleResult, cli::CrusterCLIError};

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
            return Ok(
                AuditPrintConfig {
                    audit_name,
                    all: true,
                    index: 0,
                    wout_body: false,
                    no_data: false,
                    init_data: false,
                    sort: false,
                    sort_ruleid: false,
                    sort_severity: false
                }
            )
        }
        else {
            let index = value.get_one::<usize>("index").unwrap().to_owned();
            let wout_body = value.get_flag("without-body");
            let init_data = value.get_flag("initial-data");
            let no_data = value.get_flag("no-data");
            let sort_severity = value.get_flag("sort-severity");
            let sort_ruleid = value.get_flag("sort-ruleid");
            let sort = sort_severity || sort_ruleid;

            return Ok(
                AuditPrintConfig {
                    audit_name,
                    all: false,
                    index,
                    wout_body,
                    no_data,
                    init_data,
                    sort,sort_severity,
                    sort_ruleid,
                }
            )
        }
    }
}

fn print_http_message<T: Display>(shift: &str, label: T, data: &str, wout_body: bool) {
    println!("{}{}\n", shift, label);
                                
    let splitted_request: Vec<&str> = data.split("\n").collect();
    for line in splitted_request {
        if line == "\r" && wout_body {
            println!("{}\t >", shift);
            break;
        }

        println!("{}\t > {}", shift, line);
    }

    println!("");
}

fn print_single_result(print_conf: &AuditPrintConfig, line: String) -> Result<bool, CrusterCLIError> {
    let finding = serde_json::from_str::<RuleResult>(&line)?;
                    
    if finding.get_id() != print_conf.index {
        return Ok(false);
    }

    println!("{:<10}  {:<}", "Rule ID:", finding.get_rule_id());
    println!("{:<10}  {:<}", "Severity:", finding.get_severity());
    println!("{:<10}  {:<}", "Protocol:", finding.get_protocol());
    println!("{:<10}  {:<}", "Type:", finding.get_type());
    println!("{:<10}  {:<}", "About:", finding.get_about());

    let actual_findings = finding.get_findings();
    println!("\nFindings:");
    for (finding_name, (extracted, send_results)) in actual_findings.iter() {
        let joined_extracted_items = extracted.join(", ");

        println!("\t{:<10}:  {:<}", "Name", finding_name);
        println!("\t{:<10}:  {:<}", "Extracted", joined_extracted_items);
        println!("");

        if !print_conf.no_data {
            for send_result in send_results {

                print_http_message(
                    "\t",
                    format!("Request (payload='{}'):", &send_result.payload),
                    &send_result.request,
                    print_conf.wout_body
                );

                print_http_message(
                    "\t",
                    format!("Response (payload='{}'):", &send_result.payload),
                    &send_result.response,
                    print_conf.wout_body
                );

            }

            if print_conf.init_data {
                
                print_http_message(
                    "",
                    "Initial request:",
                    finding.get_initial_request(),
                    print_conf.wout_body
                );

                print_http_message(
                    "",
                    "Initial response:",
                    finding.get_initial_response(),
                    print_conf.wout_body
                );

            }
        }      
    }

    Ok(true)
}

fn print_finding_briefly(finding: &RuleResult) {
    let all_findings = finding.get_all_findings_as_str();
    let all_findings_cutted = if all_findings.len() > 69 {
        &all_findings[..69]
    }
    else {
        &all_findings
    };

    println!(
        "{:>4}  {:<8} {:<30}  {:<70}  {:<}",
        finding.get_id(),
        finding.get_severity(),
        finding.get_rule_id(),
        all_findings_cutted,
        finding.get_initial_request_first_line()
    );
}

fn print_all(print_conf: &AuditPrintConfig, reader: BufReader<fs::File>) -> Result<(), CrusterCLIError> {
    let mut lines: Vec<RuleResult> = vec![];
    for possible_line in reader.lines() {
        match possible_line {
            Ok(line) => {
                let finding = serde_json::from_str::<RuleResult>(&line)?;

                if print_conf.sort {
                    lines.push(finding);
                    continue;
                }
                else {
                    print_finding_briefly(&finding);
                }
            },
            Err(err) => {
                return Err(CrusterCLIError::from(err));
            }
        }
    }

    if !lines.is_empty() {
        if print_conf.sort_ruleid {
            lines.sort_by(|a, b| { a.get_rule_id().cmp(b.get_rule_id()) });
        }
        
        if print_conf.sort_severity {
            lines.sort_by(|a, b| { a.get_num_severity().cmp(&b.get_num_severity()) });
        }

        for line in lines.iter() {
            print_finding_briefly(line);
        }
    }
    
    Ok(())
}

pub(crate) async fn exec(print_conf: AuditPrintConfig, results: String) -> Result<(), CrusterCLIError> {
    let fin = fs::OpenOptions::new().read(true).open(&results)?;
    let reader = BufReader::new(fin);

    if print_conf.all {
        print_all(&print_conf, reader)?;
    }
    else {
        for possible_line in reader.lines() {
            match possible_line {
                Ok(line) => {
                    if print_single_result(&print_conf, line)? {
                        return Ok(());
                    }
                },
                Err(err) => {
                    return Err(CrusterCLIError::from(err));
                }
            }
        }

        println!("Cannot get finding with index {} in results of audit '{}'", print_conf.index, &print_conf.audit_name);
    }

    Ok(())
}