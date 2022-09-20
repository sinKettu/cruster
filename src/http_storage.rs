use std::cmp::min;
use std::collections::HashMap;
use http::header::HeaderName;
use http::HeaderValue;

use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};
use super::ui::ui_storage::DEFAULT_TABLE_WINDOW_SIZE;

use regex;
use crate::CrusterError;

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

impl CacheKey {
    fn filter_exists(&self) -> bool {
        return match self.filter {
            Some(_) => {
                true
            },
            None => {
                false
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) struct HTTPStorage {
    pub(super) storage: Vec<RequestResponsePair>,
    context_reference: HashMap<usize, (usize, bool)>,
    cache_key: CacheKey,
    cache_buffer: Vec<usize>,
    filtered_ref: Vec<usize>
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
            filtered_ref: Vec::new(),
            // seek: 0,
            // capacity: 10000
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: usize) {
        let index = self.storage.len();
        self.storage.push(
            RequestResponsePair {
                request: Some(request),
                response: None,
                index,
            }
        );

        let match_filter = if self.cache_key.filter_exists() {
            // filter existing guarantees that we can unwrap
            let re = regex::Regex::new(self.cache_key.filter.as_ref().unwrap()).unwrap();
            if self.filter(&re, &self.storage[index]) {
                self.filtered_ref.push(index);
                true
            }
            else {
                false
            }
        }
        else {
            false
        };

        self.context_reference.insert(addr, (index, match_filter));
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) {
        if let Some((index, match_filter)) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);

            if ! match_filter.to_owned() && self.cache_key.filter_exists() {
                let re = regex::Regex::new(self.cache_key.filter.as_ref().unwrap()).unwrap();
                if self.filter(&re, &self.storage[index.to_owned()]) {
                    self.filtered_ref.push(index.to_owned());
                }
            }
        }

        self.context_reference.remove(addr);
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }

    pub(crate) fn cache_len(&self) -> usize {
        return self.cache_buffer.len().clone();
    }

    pub(crate) fn get_cached_data(&mut self, skip_count: usize, take_count: usize, filter_re: Option<regex::Regex>, force: bool) -> Vec<&RequestResponsePair> {
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

        if current_cache_key != self.cache_key || force {
            // if filter (regex) changed we have to update self.filtered_ref
            match (current_cache_key.filter.as_ref(), self.cache_key.filter.as_ref()) {
                (Some(current), Some(persisted)) => {
                    if current.as_str() != persisted.as_str() {
                        let re = regex::Regex::new(current_cache_key.filter.as_ref().unwrap()).unwrap();
                        self.update_filtered_reference(Some(&re));
                    }
                },
                (None, Some(_)) => {
                    self.update_filtered_reference(None);
                },
                (Some(_), None) => {
                    let re = regex::Regex::new(current_cache_key.filter.as_ref().unwrap()).unwrap();
                    self.update_filtered_reference(Some(&re))
                }
                _ => {}
            }

            self.cache_key = current_cache_key;

            self.cache_buffer = if ! self.cache_key.filter_exists(){
                self.storage
                    .iter()
                    .enumerate()
                    .skip(self.cache_key.skip)
                    .take(self.cache_key.take)
                    .map(|(idx, elem)| { idx })
                    .collect()
            }
            else {
                self.filtered_ref
                    .iter()
                    .skip(self.cache_key.skip)
                    .take(self.cache_key.take)
                    .map(|idx| {
                        idx.clone()
                    })
                    .collect()
            }
        }

        return self.cache_buffer
            .iter()
            .map(|elem| {
                &self.storage[elem.clone()]
            })
            .collect::<Vec<&RequestResponsePair>>();
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

    pub(crate) fn get_pair_from_cache(&self, idx: usize) -> Result<&RequestResponsePair, CrusterError> {
        return if idx < self.cache_buffer.len() {
            Ok(&self.storage[self.cache_buffer[idx]])
        }
        else {
            Err(
                CrusterError::ProxyTableIndexOutOfRange(
                    format!(
                        "Requested element with number {} from table cache buffer with length {}",
                        idx,
                        self.cache_buffer.len()
                    )
                )
            )
        }
    }

    fn update_filtered_reference(&mut self, re: Option<&regex::Regex>) {
        self.filtered_ref.clear();
        if re.is_none() {
            return;
        }

        for (idx, pair) in self.storage.iter().enumerate() {
            if self.filter(re.unwrap(), pair) {
                self.filtered_ref.push(idx);
            }
        }
    }

    pub(crate) fn actual_len(&self) -> usize {
        if self.filtered_ref.len() > 0 {
            self.filtered_ref.len().clone()
        }
        else {
            self.storage.len()
        }
    }
}
