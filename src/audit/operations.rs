use crate::{audit::RuleResult, http_storage::serializable::SerializableProxyData};
use std::{
    collections::HashMap,
    io::{self, BufRead},
    sync::Arc,
};

use crate::{config::AuditConfig, http_storage::RequestResponsePair};
use regex::Regex;

pub(crate) fn compile_pair_filters(
    config: &AuditConfig,
) -> Result<Vec<Regex>, crate::audit::AuditError> {
    config
        .filter
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|pattern| {
            Regex::new(pattern).map_err(|err| {
                crate::audit::AuditError::from(format!(
                    "Invalid audit HTTP pair filter '{}': {}",
                    pattern, err
                ))
            })
        })
        .collect()
}

pub(crate) fn filter_pairs(pairs: &mut Vec<Arc<RequestResponsePair>>, filters: &[Regex]) {
    if !filters.is_empty() {
        pairs.retain(|pair| {
            filters.iter().any(|re| {
                pair.request.as_ref().is_some_and(|r| r.serach_with_re(re))
                    || pair.response.as_ref().is_some_and(|r| r.serach_with_re(re))
            })
        });
    }
}

pub(crate) fn read_request_urls(reader: impl BufRead) -> io::Result<HashMap<usize, String>> {
    reader
        .lines()
        .map(|line| {
            let pair: SerializableProxyData = serde_json::from_str(&line?)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
            Ok((pair.index, pair.request.get_uri()))
        })
        .collect()
}

pub(crate) fn request_urls(
    records: impl IntoIterator<Item = SerializableProxyData>,
) -> HashMap<usize, String> {
    records
        .into_iter()
        .map(|pair| (pair.index, pair.request.get_uri()))
        .collect()
}

#[derive(Default)]
pub(crate) struct FindingQuery {
    pub(crate) id: Option<usize>,
    pub(crate) pair_ids: Option<Vec<usize>>,
    pub(crate) rule_ids: Option<Vec<String>>,
    pub(crate) severities: Option<Vec<String>>,
    pub(crate) filter: Option<Regex>,
    pub(crate) sort_ruleid: bool,
    pub(crate) sort_severity: bool,
}

/// CLI's stable sort: rule ID first, then severity, preserving file order on ties.
fn sort_findings(results: &mut [RuleResult], query: &FindingQuery) {
    if query.sort_ruleid {
        results.sort_by(|a, b| a.get_rule_id().cmp(b.get_rule_id()));
    }
    if query.sort_severity {
        results.sort_by_key(RuleResult::get_num_severity);
    }
}

/// Shared domain selection. Unsorted reads stay lazy: a single-ID lookup or a
/// broken terminal pipe must not read/validate the remainder of the file.
/// Regexes deliberately match original JSONL, before any output projection.
pub(crate) fn query_findings<'a>(
    reader: impl BufRead + 'a,
    query: FindingQuery,
) -> io::Result<Box<dyn Iterator<Item = io::Result<RuleResult>> + 'a>> {
    let sorted = query.sort_ruleid || query.sort_severity;
    let sort_query = FindingQuery {
        sort_ruleid: query.sort_ruleid,
        sort_severity: query.sort_severity,
        ..Default::default()
    };
    let results = reader.lines().filter_map(move |line| {
        let parsed = (|| {
            let line = line?;
            let result: RuleResult = serde_json::from_str(&line)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
            if query.id.is_some_and(|id| id != result.get_id())
                || query
                    .pair_ids
                    .as_ref()
                    .is_some_and(|ids| !ids.contains(&result.get_pair_index()))
                || query
                    .rule_ids
                    .as_ref()
                    .is_some_and(|ids| !ids.iter().any(|id| id == result.get_rule_id()))
                || query
                    .severities
                    .as_ref()
                    .is_some_and(|items| !items.iter().any(|s| s == result.get_severity()))
                || query.filter.as_ref().is_some_and(|re| !re.is_match(&line))
            {
                Ok(None)
            } else {
                Ok(Some(result))
            }
        })();
        match parsed {
            Ok(Some(result)) => Some(Ok(result)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    });
    if sorted {
        let mut results = results.collect::<io::Result<Vec<_>>>()?;
        sort_findings(&mut results, &sort_query);
        Ok(Box::new(results.into_iter().map(Ok)))
    } else {
        Ok(Box::new(results))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn finding(id: usize, rule: &str, severity: &str) -> String {
        json!({"id":id,"rule_id":rule,"about":"fixture","protocol":"http","type":"passive",
            "pair_index":42,"severity":severity,"findings":{},"initial_request":"GET / HTTP/1.1\r\n\r\nsecret",
            "initial_response":"HTTP/1.1 200 OK\r\n\r\n"}).to_string()
    }

    #[test]
    fn shared_query_keeps_cli_stable_sort_and_matches_before_projection() {
        let text = [
            (0, "z", "high"),
            (1, "b", "info"),
            (2, "a", "info"),
            (3, "a", "info"),
        ]
        .into_iter()
        .map(|(id, rule, severity)| finding(id, rule, severity))
        .collect::<Vec<_>>()
        .join("\n");
        let query = FindingQuery {
            pair_ids: Some(vec![42]),
            severities: Some(vec!["info".into()]),
            filter: Some(Regex::new("secret").unwrap()),
            sort_ruleid: true,
            sort_severity: true,
            ..Default::default()
        };
        let results = query_findings(text.as_bytes(), query)
            .unwrap()
            .collect::<io::Result<Vec<_>>>()
            .unwrap();
        assert_eq!(
            results.iter().map(RuleResult::get_id).collect::<Vec<_>>(),
            [2, 3, 1]
        );
    }

    #[test]
    fn shared_query_preserves_single_lookup_early_exit_and_reports_later_errors() {
        let text = format!("{}\ninvalid json\n", finding(7, "rule", "info"));
        let mut results = query_findings(
            text.as_bytes(),
            FindingQuery {
                id: Some(7),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(results.next().unwrap().unwrap().get_id(), 7);
        assert_eq!(
            results.next().unwrap().err().unwrap().kind(),
            io::ErrorKind::InvalidData
        );
        assert!(query_findings(
            text.as_bytes(),
            FindingQuery {
                sort_severity: true,
                ..Default::default()
            }
        )
        .is_err());
    }
}
