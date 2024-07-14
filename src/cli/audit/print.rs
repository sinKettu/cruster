use std::{fs::{self, File}, io::{BufRead, BufReader}};

use clap::ArgMatches;

use crate::{audit::RuleResult, cli::CrusterCLIError};

pub(crate) struct AuditPrintConfig {
    pub(crate) audit_name: String,
    pub(crate) all: bool,
    pub(crate) index: usize,
    pub(crate) wout_body: bool,
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
                    wout_body: false
                }
            )
        }
        else {
            let index = value.get_one::<usize>("index").unwrap().to_owned();
            let wout_body = value.get_flag("without-body");

            return Ok(
                AuditPrintConfig {
                    audit_name,
                    all: false,
                    index,
                    wout_body
                }
            )
        }
    }
}

pub(crate) async fn exec(print_conf: AuditPrintConfig, results: String) -> Result<(), CrusterCLIError> {
    let fin = fs::OpenOptions::new().read(true).open(&results)?;
    let reader = BufReader::new(fin);

    if print_conf.all {
        for possible_line in reader.lines() {
            match possible_line {
                Ok(line) => {
                    let finding = serde_json::from_str::<RuleResult>(&line)?;
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
                },
                Err(err) => {
                    return Err(CrusterCLIError::from(err));
                }
            }
        }
    }
    else {
        let mut found = false;
        for possible_line in reader.lines() {
            match possible_line {
                Ok(line) => {
                    let finding = serde_json::from_str::<RuleResult>(&line)?;
                    
                    if finding.get_id() != print_conf.index {
                        continue;
                    }

                    found = true;

                    println!("{:<10}  {:<}", "Rule ID:", finding.get_rule_id());
                    println!("{:<10}  {:<}", "Severity:", finding.get_severity());

                    let actual_findings = finding.get_findings();
                    println!("\nFindings:");
                    for (finding_name, (extracted, _send_results)) in actual_findings.iter() {
                        let joined_extracted_items = extracted.join(", ");
                        println!("\t{:<10}:  {:<}", finding_name, joined_extracted_items);
                    }
                },
                Err(err) => {
                    return Err(CrusterCLIError::from(err));
                }
            }
        }

        if !found {
            println!("Cannot get finding with index {} in results of audit '{}'", print_conf.index, &print_conf.audit_name);
        }
    }

    Ok(())
}