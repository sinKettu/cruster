pub(crate) mod write;

use std::{collections::HashMap, fmt::Display};

use super::{types::SerializableSendResultEntry, AuditError, RuleResult, RuleSeverity};

use colored::Colorize;


impl Display for RuleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (severity, shift) = match &self.severity {
            RuleSeverity::Info => { ("info".cyan(), 2) },
            RuleSeverity::Low => { ("low".bright_green(), 3) },
            RuleSeverity::Medium => { ("medium".bright_yellow(), 0) },
            RuleSeverity::High => { ("high".bright_red(), 2) }
        };

        write!(f, "{}[{}] {} - {}: ", " ".repeat(shift), severity, self.pair_index, &self.rule_id.green())?;
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

        let initial_request = self.get_initial_request()
            .split("\n")
            .collect::<Vec<&str>>()[0]
                .trim();

        write!(f, " > {}", initial_request.bright_black())?;

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

    // for sorting
    pub(crate) fn get_num_severity(&self) -> u8 {
        return match self.severity {
            RuleSeverity::High => 4,
            RuleSeverity::Medium => 3,
            RuleSeverity::Low => 2,
            RuleSeverity::Info => 1,
        }
    }

    pub(crate) fn get_protocol(&self) -> &str {
        return &self.protocol
    }

    pub(crate) fn get_type(&self) -> &str {
        return &self.r#type
    }

    pub(crate) fn get_rule_id(&self) -> &str {
        return &self.rule_id;
    }

    pub(crate) fn get_about(&self) -> &str {
        return &self.about
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

    pub(crate) fn get_initial_request(&self) -> &str {
        return &self.initial_request
    }

    pub(crate) fn get_initial_response(&self) -> &str {
        return &self.initial_response
    }
}

pub fn syntax_string() -> String {
    format!("[{}] {} - {}: {} ({}, {}) / {} () / ...", "severity".cyan(), "pair_index", "rule_id".green(), "finding_name_1".bright_blue(), "extracted1".bold(), "extracted2".bold(), "finding_name_2".bright_blue())
}

pub(crate) trait WriteResult {
    fn write_result(&self, filename: &str) -> Result<(), AuditError>;
}