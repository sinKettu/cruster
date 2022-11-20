use std::cmp::min;
use std::collections::HashMap;
use http::header::HeaderName;
use http::HeaderValue;

use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

use regex;
use crate::CrusterError;
use crate::siv_ui::ProxyDataForTable;

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
    // cache_key: CacheKey,
    // cache_buffer: Vec<usize>,
    // filtered_ref: Vec<usize>
    // seek: usize,
    // capacity: usize
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::with_capacity(1000),
            context_reference: HashMap::new(),
            // cache_key: CacheKey::default(),
            // cache_buffer: Vec::new(),
            // filtered_ref: Vec::new(),
            // seek: 0,
            // capacity: 10000
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: usize) -> ProxyDataForTable {
        let index = self.storage.len();

        let table_record = ProxyDataForTable {
            id: index,
            hostname: request.get_host(),
            path: request.get_request_path(),
            method: request.method.clone(),
        };

        self.storage.push(
            RequestResponsePair {
                request: Some(request),
                response: None,
                index,
            }
        );

        self.context_reference.insert(addr, (index, true));
        return table_record;
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) {
        if let Some((index, match_filter)) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);

            // if ! match_filter.to_owned() && self.cache_key.filter_exists() {
            //     let re = regex::Regex::new(self.cache_key.filter.as_ref().unwrap()).unwrap();
            //     if self.filter(&re, &self.storage[index.to_owned()]) {
            //         self.filtered_ref.push(index.to_owned());
            //     }
            // }
        }

        self.context_reference.remove(addr);
    }

    pub(crate) fn get(&self, idx: usize) -> &RequestResponsePair {
        &self.storage[idx]
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }
}
