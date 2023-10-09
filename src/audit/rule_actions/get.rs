use std::collections::HashMap;

use super::*;

impl RuleGetAction {
    pub(crate) fn check_up(&mut self, possible_find_ref: Option<&HashMap<String, usize>>) -> Result<(), AuditError> {
        if let Ok(num_find_id) = self.find_id.parse::<usize>() {
            self.find_id_cache = Some(num_find_id)
        }
        else {
            if let Some(find_ref) = possible_find_ref {
                if let Some(num_find_id) = find_ref.get(&self.find_id) {
                    self.find_id_cache = Some(num_find_id.to_owned());
                }
                else {
                    return Err(
                        AuditError(format!("could not find a FIND action with id '{}'", &self.find_id))
                    );
                }
            }
            else {
                return Err(
                    AuditError(format!("FIND action with id '{}' cannot be found", &self.find_id))
                );
            }
        }

        match self.extract.as_str() {
            "LINE" => {
                self.extract_cache = Some(ExtractionMode::LINE)
            },
            "MATCH" => {
                self.extract_cache = Some(ExtractionMode::MATCH)
            },
            _ => {
                self.extract_cache = Some(ExtractionMode::GROUP(self.extract.clone()))
            }
        }

        Ok(())
    }
}
