use std::collections::HashMap;

use bstr::ByteSlice;
use log::debug;

use crate::audit::{contexts::traits::{WithWatchAction}, types::{CapturesBorders, SingleCoordinates}};

use super::*;

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

    pub(crate) fn exec<'pair_lt, 'rule_lt, T: WithWatchAction<'pair_lt>>(&self, ctxt: &mut T) -> Result<(), AuditError> {
        let Some(request) = ctxt.initial_request() else {
            let err_str = format!("HTTP pair with id {} has empty request", ctxt.pair_id());
            return Err(AuditError(err_str));
        };

        let byte_re = match regex::bytes::Regex::new(&self.pattern) {
            Ok(re) => {
                re
            },
            Err(err) => {
                let err_str = format!("Could not parse pattern (for bytes) '{}': {}", &self.pattern, err);
                return Err(AuditError(err_str));
            }
        };

        debug!("WatchAction - byte regex compiled: {:?}", &byte_re);

        let insert_closure = |line_index: usize, line: &[u8], substring_offset: usize, byte_re: &regex::bytes::Regex| -> CapturesBorders {
            let mut cap_borders: CapturesBorders = HashMap::default();

            let Some(cptrs) = byte_re.captures(line) else {
                return cap_borders;
            };

            // debug!("{:#?}", cptrs);

            for (cname_index, cname) in byte_re.capture_names().enumerate() {
                match cname {
                    Some(cname) => {
                        let Some(mtch) = cptrs.name(cname) else {
                            continue;
                        };

                        let coordinates = SingleCoordinates {
                            line: line_index,
                            start: mtch.start() + substring_offset,
                            end: mtch.end() + substring_offset
                        };

                        cap_borders.insert(cname.to_string(), vec![coordinates]);
                    },
                    None => {
                        let Some(mtch) = cptrs.get(cname_index) else {
                            continue;
                        };

                        let coordinates = SingleCoordinates {
                            line: line_index,
                            start: mtch.start() + substring_offset,
                            end: mtch.end() + substring_offset
                        };

                        cap_borders.insert(cname_index.to_string(), vec![coordinates]);
                    }
                }
            }

            return cap_borders;
        };

        let captures_borders: CapturesBorders = match self.part_cache.as_ref().unwrap() {
            WatchPart::Method => {
                debug!("WatchAction - watching in request method");
                insert_closure(0, request.method.as_bytes(), 0, &byte_re)
            },
            WatchPart::Headers => {
                debug!("WatchAction - watching in request headers");
                let mut captures_borders: CapturesBorders = HashMap::default();
                for (h_index, (key, value)) in request.headers.iter().enumerate() {
                    let str_for_search = [key.as_str().as_bytes(), b": ", value.as_bytes()].concat();
                    debug!("WatchAction - checking header string: {}", str_for_search.as_slice().to_str_lossy());
                    let sub_results = insert_closure(h_index + 1, &str_for_search, 0, &byte_re);

                    for (cname, borders) in sub_results {
                        debug!("WatchAction - found capture with group name '{}', coordinates: {:?}", &cname, &borders);
                        if captures_borders.contains_key(&cname) {
                            let mut initial_results = captures_borders.remove(&cname).unwrap();
                            initial_results.extend(borders);
                            captures_borders.insert(cname, initial_results);
                        }
                        else {
                            captures_borders.insert(cname, borders);
                        }
                    }
                }

                captures_borders
            },
            WatchPart::Path => {
                debug!("WatchAction - watching in request path (including query and fragment)");
                let pth = request.get_request_path();
                let offset = request.method.as_bytes().len() + 1;
                insert_closure(0, pth.as_bytes(), offset, &byte_re)
            },
            WatchPart::Version => {
                debug!("WatchAction - watching in request version");
                let offset = request.method.as_bytes().len() + 1 + request.get_request_path().as_bytes().len() + 1;
                insert_closure(0, request.version.as_bytes(), offset, &byte_re)
            },
            WatchPart::Body => {
                debug!("WatchAction - watching in request body");
                let lines_count_start = 1 + request.headers.len();
                // I'm not sure how this will work in case of utf-*
                let body_lines = request.body.split(|chr| { (*chr as char) == '\n' });
                let mut captures_borders: CapturesBorders = HashMap::default();

                for (index, body_line) in body_lines.enumerate() {
                    debug!("WatchAction - checking body line: {}", body_line.to_str_lossy());
                    let sub_results = insert_closure(lines_count_start + index, &body_line, 0, &byte_re);

                    for (cname, borders) in sub_results {
                        debug!("WatchAction - found capture with group name '{}', coordinates: {:?}", &cname, &borders);
                        if captures_borders.contains_key(&cname) {
                            let mut initial_results = captures_borders.remove(&cname).unwrap();
                            initial_results.extend(borders);
                            captures_borders.insert(cname, initial_results);
                        }
                        else {
                            captures_borders.insert(cname, borders);
                        }
                    }
                }

                captures_borders
            },
        };

        debug!("WatchAction - Final result: {:?}", &captures_borders);
        ctxt.add_watch_result(captures_borders);
        Ok(())
    }
}
