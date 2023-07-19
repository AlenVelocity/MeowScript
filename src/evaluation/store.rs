use super::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(PartialEq, Clone, Debug)]
pub struct Store {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Store>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn from(store: HashMap<String, Object>) -> Self {
        Self { store, outer: None }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Store>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(e) => Some(e).cloned(),
            None => {
                if let Some(ref o) = self.outer {
                    return o.borrow_mut().get(name);
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Option<Object> {
        self.store.insert(name, val)
    }

    pub fn anew(&mut self, name: String, val: Object) -> Option<Object> {
        match self.store.get(&name) {
            Some(_e) => self.store.insert(name, val),
            None => {
                if let Some(ref o) = self.outer {
                    let mut outer = o.borrow_mut();
                    match outer.get(&name) {
                        Some(_) => outer.anew(name, val),
                        None => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn iter (&self) -> std::collections::hash_map::Iter<String, Object> {
        self.store.iter()
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}