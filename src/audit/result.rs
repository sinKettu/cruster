pub(crate) mod write;

use std::{collections::HashMap, fmt::Display};

use super::{types::SerializableSendResultEntry, AuditError, RuleResult, RuleSeverity};

use colored::Colorize;


impl Display for RuleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match &self.severity {
            RuleSeverity::Info => { "info".cyan() },
            RuleSeverity::Low => { "low".bright_green() },
            RuleSeverity::Medium => { "medium".bright_yellow() },
            RuleSeverity::High => { "high".bright_red() }
        };

        write!(f, "[{}] {} - {}: ", severity, self.pair_index, &self.rule_id.green())?;
        let mut first_finding = true;

        for (found, extracted) in self.findings.iter() {
            if first_finding {
                first_finding = false;
            }
            else {
                write!(f, " / ")?;
            }

            write!(f, "{} (", found.bright_blue())?;            

            let mut first_extracted_item = true;
            for extracted_item in extracted.0.iter() {
                if first_extracted_item {
                    first_extracted_item = false;
                }
                else {
                    write!(f, ", ")?;
                }

                write!(f, "{}", extracted_item.bold())?;
            }

            write!(f, ")")?;
        }

        Ok(())
    }
}

impl RuleResult {
    pub(crate) fn get_id(&self) -> usize {
        self.id.to_owned()
    }

    pub(crate) fn set_id(&mut self, new_id: usize) {
        self.id = new_id
    }

    pub(crate) fn get_severity(&self) -> &str {
        return match self.severity {
            RuleSeverity::High => "high",
            RuleSeverity::Medium => "medium",
            RuleSeverity::Low => "low",
            RuleSeverity::Info => "info",
        }
    }

    pub(crate) fn get_rule_id(&self) -> &str {
        return &self.rule_id;
    }

    pub(crate) fn get_all_findings_as_str(&self) -> String {
        let mut result = String::default();
        let mut first_finding = true;

        for (finding_name, (extracted, _)) in self.findings.iter() {
            if first_finding {
                first_finding = false;
            }
            else {
                result.push_str(" / ");
            }

            result.push_str(finding_name);
            result.push_str("(");

            let mut first_extracted = true;
            for extracted_item in extracted {
                if first_extracted {
                    first_extracted = false;
                }
                else {
                    result.push_str(",");
                }

                result.push_str(extracted_item);
            }
            result.push_str(")");
        }

        result
    }

    pub(crate) fn get_initial_request_first_line(&self) -> &str {
        let splitted: Vec<&str> = self.initial_request.splitn(2, "\n").collect();
        return splitted[0];
    }

    pub(crate) fn get_findings(&self) -> &HashMap<String, (Vec<String>, Vec<SerializableSendResultEntry>)> {
        return &self.findings;
    }
}

pub fn syntax_string() -> String {
    format!("[{}] {} - {}: {} ({}, {}) / {} () / ...", "severity".cyan(), "pair_index", "rule_id".green(), "finding_name_1".bright_blue(), "extracted1".bold(), "extracted2".bold(), "finding_name_2".bright_blue())
}

pub(crate) trait WriteResult {
    fn write_result(&self, filename: &str) -> Result<(), AuditError>;
}