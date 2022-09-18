use std::collections::HashMap;

use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};
use super::ui::ui_storage::DEFAULT_TABLE_WINDOW_SIZE;

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
        self.storage.push(
            RequestResponsePair {
                request: Some(request),
                response: None,
                index: self.storage.len(),
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

    pub(crate) fn get_cached_data(&mut self, skip_count: usize, take_count: usize, filter: Option<String>, force: bool) -> Vec<RequestResponsePair> {
        let current_cache_key = CacheKey {
            skip: skip_count,
            take: take_count,
            filter
        };

        if current_cache_key != self.cache_key || force {
            self.cache_key = current_cache_key;
            self.cache_buffer = self.storage
                .iter()
                .enumerate()
                .filter(|(idx, elem)| { true })
                .skip(self.cache_key.skip)
                .take(self.cache_key.take)
                .map(|(idx, elem)| { idx })
                .collect();
        }

        return self.cache_buffer
            .iter()
            .map(|elem| {
                self.storage[elem.clone()].clone()
            })
            .collect::<Vec<RequestResponsePair>>();
    }
}
