use std::{fs, io::Write};

use serde_json;

use crate::audit::{AuditError, RuleResult};
use super::WriteResult;

impl WriteResult for RuleResult {
    fn write_result(&self, filename: &str) -> Result<(), AuditError> {
        let mut fout = fs::OpenOptions::new().append(true).open(filename)?;
        let serialized_result: String = serde_json::to_string(&self)?;
        fout.write(serialized_result.as_bytes())?;
        fout.write("\n".as_bytes())?;
        
        Ok(())
    }
}