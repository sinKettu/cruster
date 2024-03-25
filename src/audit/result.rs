use std::fmt::Display;

use super::{RuleResult, RuleSeverity};

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
            write!(f, "{} (", found.bright_blue())?;

            if first_finding {
                first_finding = false;
            }
            else {
                write!(f, " / ")?;
            }

            let mut first_extracted_item = true;
            for extracted_item in extracted.iter() {
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

pub fn syntax_string() -> String {
    format!("[{}] {} - {}: {} ({}, {}) / {} () / ...", "severity".cyan(), "pair_index", "rule_id".green(), "finding_name_1".bright_blue(), "extracted1".bold(), "extracted2".bold(), "finding_name_2".bright_blue())
}
