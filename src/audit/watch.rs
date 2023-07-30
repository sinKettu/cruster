use serde::{Serialize, Deserialize};

use super::AuditError;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum WatchPart {
    Method,
    Path,
    Version,
    Headers,
    Body
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) struct RuleWatchAction {
    id: Option<String>,
    part: String,
    // This field will store more convinient representation of part after first check
    part_cache: Option<WatchPart>,
    pattern: String
}

impl RuleWatchAction {
    pub(crate) fn check_up(&mut self) -> Result<(), AuditError> {
        let lowercase_part = self.part.to_lowercase();
        self.part_cache = match lowercase_part.as_str() {
            "method" => { Some(WatchPart::Method) },
            "path" => { Some(WatchPart::Path) },
            "version" => { Some(WatchPart::Version) },
            "headers" => { Some(WatchPart::Headers) },
            "body" => { Some(WatchPart::Body) },
            _ => {
                return Err(AuditError(format!("Unknown part of HTTP request to watch for patter: {}", &self.part)));
            },
        };

        Ok(())
    }

    pub(crate) fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
}