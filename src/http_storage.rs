mod serializable;

use std::collections::HashMap;

use crate::siv_ui::ProxyDataForTable;
use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

#[derive(Clone, Debug)]
pub(super) struct RequestResponsePair {
    pub(super) request: Option<HyperRequestWrapper>,
    pub(super) response: Option<HyperResponseWrapper>,
    pub(super) index: usize,
}

pub(super) struct HTTPStorageIterator<'a> {
    object: &'a HTTPStorage,
    counter: usize
}

#[derive(Clone)]
pub(crate) struct HTTPStorage {
    storage: Vec<RequestResponsePair>,
    context_reference: HashMap<usize, usize>,
    seq_reference: Vec<Option<usize>>,
    next_id: usize,
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::with_capacity(1000),
            context_reference: HashMap::new(),
            seq_reference: vec![None; 2000],
            next_id: 0,
        }
    }
}

impl<'a> Iterator for HTTPStorageIterator<'a> {
    type Item = &'a RequestResponsePair;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.object.storage.get(self.counter);
        self.counter += 1;
        tmp
    }
}

impl<'a> IntoIterator for &'a HTTPStorage {
    type Item = &'a RequestResponsePair;
    type IntoIter = HTTPStorageIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HTTPStorageIterator {
            object: self,
            counter: 0
        }
    }
}

impl HTTPStorage {
    pub(crate) fn put_request(&mut self, request: HyperRequestWrapper, addr: usize) -> ProxyDataForTable {
        let index = self.storage.len();

        let table_record = ProxyDataForTable {
            id: self.next_id.clone(),
            hostname: request.get_host(),
            path: request.get_request_path(),
            method: request.method.clone(),
            status_code: String::default(),
            response_length: 0,
        };

        let pair = RequestResponsePair {
            request: Some(request),
            response: None,
            index,
        };

        self.insert_with_id(pair);
        self.context_reference.insert(addr, index);
        return table_record;
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) -> Option<usize> {
        let id = if let Some(index) = self.context_reference.get(addr) {
            self.storage[index.to_owned()].response = Some(response);
            let id = self.storage[index.to_owned()].index;
            Some(id)
        }
        else {
            None
        };

        if id.is_some() {
            let _result = self.context_reference.remove(addr);
        }
        
        return id;
    }

    pub(crate) fn get(&self, idx: usize) -> &RequestResponsePair {
        &self.storage[idx]
    }

    pub(crate) fn get_by_id(&self, id: usize) -> Option<&RequestResponsePair> {
        if id >= self.seq_reference.len() {
            return None;
        }

        if let Some(index) = self.seq_reference[id] {
            return Some(&self.storage[index]);
        }
        else {
            return None;
        }
    }

    fn insert_with_id(&mut self, mut pair: RequestResponsePair) {
        let id = self.next_id;
        pair.index = id;
        self.insert_with_explicit_id(id, pair);
    }

    fn insert_with_explicit_id(&mut self, id: usize, pair: RequestResponsePair) {
        if id >= self.seq_reference.len() {
            let placeholder: Option<usize> = None;
            self.seq_reference.resize(id * 2, placeholder);
        }

        self.seq_reference[id] = Some(self.storage.len());
        self.storage.push(pair);
        self.next_id += 1;
    }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }
}
