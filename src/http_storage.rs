use std::cmp::min;
use std::collections::HashMap;
use http::header::HeaderName;
use http::HeaderValue;

use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};
use super::ui::ui_storage::DEFAULT_TABLE_WINDOW_SIZE;

use regex;

#[derive(Clone, Debug)]
pub(super) struct RequestResponsePair {
    pub(super) request: Option<HyperRequestWrapper>,
    pub(super) response: Option<HyperResponseWrapper>,
    pub(super) index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CacheKey {
    skip: usize,
    take: usize,
    filter: Option<String>
}

impl Default for CacheKey {
    fn default() -> Self {
        CacheKey {
            skip: 0_usize,
            take: 0_usize,
            filter: None
        }
    }
}

pub(crate) struct HTTPStorage {
    pub(super) storage: Vec<RequestResponsePair>,
    context_reference: HashMap<usize, usize>,
    cache_key: CacheKey,
    cache_buffer: Vec<usize>,
    // seek: usize,
    // capacity: usize
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::with_capacity(1000),
            context_reference: HashMap::new(),
            cache_key: CacheKey::default(),
            cache_buffer: Vec::new(),
            // seek: 0,
            // capacity: 10000
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: usize) {
        let l = self.storage.len();
        self.storage.push(
            RequestResponsePair {
                request: Some(request),
                response: None,
                index: l,
            }
        );

        self.context_reference.insert(addr, self.storage.len() - 1);
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) {
        if let Some(index) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);
        }

        self.context_reference.remove(addr);
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }

    pub(crate) fn cache_len(&self) -> usize {
        return self.cache_buffer.len().clone();
    }

    pub(crate) fn get_cached_data(&mut self, skip_count: usize, take_count: usize, filter_re: Option<regex::Regex>, force: bool) -> Vec<RequestResponsePair> {
        let current_cache_key = CacheKey {
            skip: skip_count,
            take: take_count,
            filter: match &filter_re {
                Some(f) => {
                    Some(f.to_string())
                },
                None => {
                    None
                }
            }
        };

        self.cache_key = current_cache_key;
        self.cache_buffer = self.storage
            .iter()
            .enumerate()
            .filter(|(idx, elem)| {
                match &filter_re {
                    Some(re) => {
                        self.filter(re, elem)
                    }
                    None => {
                        true
                    }
                }
            })
            .skip(self.cache_key.skip)
            .take(self.cache_key.take)
            .map(|(idx, elem)| { idx })
            .collect();

        // if current_cache_key != self.cache_key || force {
        //     self.cache_key = current_cache_key;
        //     self.cache_buffer = self.storage
        //         .iter()
        //         .enumerate()
        //         .filter(|(idx, elem)| {
        //             match &filter_re {
        //                 Some(re) => {
        //                     self.filter(re, elem)
        //                 }
        //                 None => {
        //                     true
        //                 }
        //             }
        //         })
        //         .skip(self.cache_key.skip)
        //         .take(self.cache_key.take)
        //         .map(|(idx, elem)| { idx })
        //         .collect();
        // }

        return self.cache_buffer
            .iter()
            .map(|elem| {
                self.storage[elem.clone()].clone()
            })
            .collect::<Vec<RequestResponsePair>>();
    }

    fn decision_on_header(&self, k: &HeaderName, v: &HeaderValue, re: &regex::Regex) -> bool {
        let header_string = format!(
            "{}: {}",
            k.as_str(),
            v.to_str().unwrap()
        );
        let re_match = re.find(&header_string);
        if let Some(_) = re_match {
            return true;
        }

        return false;
    }

    fn filter(&self, re: &regex::Regex, pair: &RequestResponsePair) -> bool {
        // Check request w/out body
        if let Some(request) = &pair.request {
            let first_line = format!(
                "{} {} {}",
                &request.method,
                &request.uri,
                &request.version
            );

            if let Some(_) = re.find(&first_line) {
                return true;
            }

            for (k, v) in &request.headers {
                if self.decision_on_header(k, v, &re) {
                    return true
                }
            }
        }

        // Check response w/out body
        if let Some(response) = &pair.response {
            let first_line = format!(
                "{} {}",
                &response.status,
                &response.version
            );

            if let Some(_) = re.find(&first_line) {
                return true;
            }

            for (k, v) in &response.headers {
                if self.decision_on_header(k, v, &re) {
                    return true;
                }
            }
        }

        return false;
    }

    pub(crate) fn get_pair(&self, idx: usize) -> &RequestResponsePair {
        let index = min(idx, self.cache_buffer.len() - 1);
        return &self.storage[self.cache_buffer[index]];
    }
}
