use super::Id;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Container<T> {
    map: HashMap<Id, T>,
}

impl<T: PartialEq> Container<T> {
    pub fn add(&mut self, input: T) -> Id {
        //this is slow like O(N) slow but adding should not be super common well running soooo it should be fine
        // it may make initing taxing
        if let Some(id) = self.find_by_value(&input) {
            id
        } else {
            let n_id = self.map.len();
            self.map.insert(n_id, input);
            n_id
        }
    }

    pub fn find_by_value(&self, input: &T) -> Option<Id> {
        self.map.iter().find_map(|(key, val)| {
            if val == input {
                return Some(*key);
            } else {
                return None;
            }
        })
    }
}

impl<T> Container<T> {
    pub fn get(&self, id: Id) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.map.get_mut(&id)
    }
}

impl<T> Default for Container<T> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}
