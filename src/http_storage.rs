pub(crate) mod serializable;

use std::{fs::File, cmp::max};
use std::collections::HashMap;
use std::time;

use crate::{siv_ui::ProxyDataForTable, utils::CrusterError};
use super::cruster_proxy::request_response::{HyperRequestWrapper, HyperResponseWrapper};

#[derive(Clone, Debug)]
pub(super) struct RequestResponsePair {
    pub(super) request: Option<HyperRequestWrapper>,
    pub(super) response: Option<HyperResponseWrapper>,
    // must be named 'id' actually
    pub(super) index: usize,
    pub(super) timestamp: Option<time::SystemTime>,
}

pub(super) struct HTTPStorageIterator<'a> {
    object: &'a HTTPStorage,
    counter: usize
}

// #[derive(Clone)]
pub(crate) struct HTTPStorage {
    // General storage, keeps all HTTP data went through proxy
    storage: Vec<RequestResponsePair>,

    // Reference 'http_message_hash: pair_id', used to match request and response came from the proxy
    context_reference: HashMap<usize, usize>,

    // Reference between pair_id and real index of pair in storage
    seq_reference: Vec<Option<usize>>,

    // ID that will be assigned to the next HTTP pair
    next_id: usize,

    // File that could be open in dump mode to write there data on-the-fly
    file: Option<File>
}

impl Default for HTTPStorage {
    fn default() -> Self {
        HTTPStorage {
            storage: Vec::with_capacity(1000),
            context_reference: HashMap::new(),
            seq_reference: vec![None; 2000],
            next_id: 0,
            file: None
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
        let index = self.next_id.clone();

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
            timestamp: Some(time::SystemTime::now())
        };

        self.insert_with_id(pair);
        self.context_reference.insert(addr, index);
        return table_record;
    }

    pub(crate) fn put_response(&mut self, response: HyperResponseWrapper, addr: &usize) -> Option<usize> {
        let id = if let Some(index) = self.context_reference.remove(addr) {
            index
        }
        else {
            return None;
        };

        let possible_pair = self.get_mut_by_id(id);
        return match possible_pair {
            Some(pair) => {
                pair.response = Some(response);
                Some(id)
            },
            None => None
        };
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

    pub(crate) fn get_mut_by_id(&mut self, id: usize) -> Option<&mut RequestResponsePair> {
        if id >= self.seq_reference.len() {
            return None;
        }

        if let Some(index) = self.seq_reference[id] {
            return Some(&mut self.storage[index]);
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
        self.next_id = max(self.next_id, id) + 1;
    }

    // fn replace_with_explicit_id(&mut self, id: usize, pair: RequestResponsePair) -> Result<(), CrusterError> {
    //     if id >= self.seq_reference.len() {
    //         return Err(
    //             CrusterError::UndefinedError(
    //                 format!("Could not replace record with id {}, because there is no such record", id)
    //             )
    //         );
    //     }

    //     let index = self.seq_reference[id];
    //     return if let Some(index) = index {
    //         self.storage[index] = pair;
    //         Ok(())
    //     }
    //     else {
    //         Err(
    //             CrusterError::UndefinedError(
    //                 format!("Could not replace record with id {}, because there is no such record", id)
    //             )
    //         )
    //     };
    // }

    pub(crate) fn len(&self) -> usize {
        return self.storage.len().clone();
    }

    pub(crate) fn clear(&mut self, force_uncompleted: bool) -> Result<(), CrusterError> {
        while self.len() > 0 {
            let idx = self.storage[self.len() - 1].index;
            self.remove_by_id(idx, force_uncompleted)?;
        }

        Ok(())
    }

    fn swap_pairs(&mut self, left: usize, right: usize) {
        self.storage.swap(left, right);
        let (lid, rid) = (self.storage[left].index, self.storage[right].index);
        self.seq_reference.swap(lid,rid);
    }

    // fn find_next_completed(&self, idx: usize) -> Option<usize> {
    //     for (i, pair) in self.storage.iter().skip(idx).enumerate() {
    //         if pair.request.is_some() && pair.response.is_some() {
    //             return Some(i);
    //         }
    //     }

    //     return None;
    // }

    // // Place all unclompleted requests to the start of storage
    // fn restructure_storage(&mut self) {
    //     let mut uncompleted_index = self.len() - 1;

    //     let mut completed_index = if let Some(index) = self.find_next_completed(0) {
    //         index
    //     }
    //     else {
    //         return;
    //     };

    //     while completed_index > uncompleted_index {
    //         let pair = &self.storage[uncompleted_index];

    //         if pair.request.is_some() && pair.response.is_some() {
    //             uncompleted_index -= 1;
    //             continue;
    //         }

    //         self.swap_pairs(completed_index, uncompleted_index);
            
    //         uncompleted_index -= 1;
    //         completed_index = if let Some(index) = self.find_next_completed(0) {
    //             index
    //         }
    //         else {
    //             return;
    //         };
    //     }
    // }

    pub(crate) fn remove_uncompleted(&mut self, hash: usize) -> Result<(), CrusterError> {
        let id = self.context_reference.remove(&hash);

        let id = if let Some(id) = id {
            id
        }
        else {
            return Err(CrusterError::UndefinedError("Cannot remove uncompleted, hash not found".to_string()));
        };

        let index = if let Some(index) = self.seq_reference[id] {
            index
        }
        else {
            return Err(
                CrusterError::UndefinedError("Cannot remove uncompleted, record not found".to_string())
            );
        };

        if self.len() == 1 || index == self.len() - 1 {
            let _ = self.storage.pop();
            self.seq_reference[id] = None;
        }
        else if id != self.len() - 1 {
            // swap with removing element with last one and do pop
            self.swap_pairs(index, self.len() - 1);
            let _ = self.storage.pop();
            self.seq_reference[id] = None;
        }

        Ok(())
    }

    pub(crate) fn remove_uncompleted_older_than(&mut self, max_ttl: time::Duration) -> Result<(), CrusterError> {
        let mut to_remove: Vec<usize> = Vec::with_capacity(self.context_reference.len());
        for (hash, id) in self.context_reference.iter() {
            let pair = self.get_by_id(id.to_owned());
            if pair.is_none() {
                to_remove.push(hash.to_owned());
                continue;
            }

            let timestamp = pair.unwrap().timestamp.clone();
            if let Some(ts) = timestamp {
                let ttl = time::SystemTime::now().duration_since(ts)?;
                if ttl < max_ttl {
                    continue;
                }
            }

            to_remove.push(hash.to_owned());
        }

        for hash in to_remove {
            self.remove_uncompleted(hash)?;
        }

        Ok(())
    }

    pub(crate) fn remove_by_id(&mut self, id: usize, force_uncompleted: bool) -> Result<(), CrusterError> {
        if self.len() > 0 {
            let index = self.seq_reference[id];
            if let None = index {
                return Err(
                    CrusterError::UndefinedError(
                        format!("Cannot find record with id {} to remove", id)
                    )
                );
            }

            let pair = &self.storage[index.unwrap()];
            if (pair.request.is_none() || pair.response.is_none()) && ! force_uncompleted {
                return Err(
                    CrusterError::UndefinedError(
                        format!("Cannot remove record with id {}, because it is uncompleted", id)
                    )
                );
            }

            if index.unwrap() < self.len() - 1 {
                self.swap_pairs(index.unwrap(), self.len() - 1);
            }

            let _ = self.storage.pop();
            self.seq_reference[id] = None;

            return Ok(());
        }
        else {
            return Err(
                CrusterError::UndefinedError(
                    format!("Cannot pop record from HTTP storage, it is empty")
                )
            );
        }
    }

    // pub(crate) fn get_bounds(&self) -> (usize, usize) {
    //     let mut min = usize::MAX;
    //     let mut max = 0_usize;

    //     for pair in self {
    //         if pair.index < min {
    //             min = pair.index;
    //             continue;
    //         }

    //         if pair.index > max {
    //             max = pair.index;
    //             continue;
    //         }
    //     }

    //     return (min, max);
    // }
}
