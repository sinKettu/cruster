use std::fmt::Display;

use super::RuleResult;

use colored::Colorize;


impl Display for RuleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: ", &self.severity.blue(), &self.rule_id.green())?;
        for (found, extracted) in self.findings.iter() {
            write!(f, "{} (", found.yellow())?;
            let mut first = true;
            for extracted_item in extracted.iter() {
                if first {
                    first = false;
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