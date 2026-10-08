//! `scratch` defines in the innermost scope; `amew` updates the nearest scope that has the name.

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Env(Rc<RefCell<Scope>>);

struct Scope {
    vars: HashMap<Rc<str>, Value>,
    parent: Option<Env>,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Env {
        Env(Rc::new(RefCell::new(Scope {
            vars: HashMap::new(),
            parent: None,
        })))
    }

    pub fn child(&self) -> Env {
        Env(Rc::new(RefCell::new(Scope {
            vars: HashMap::new(),
            parent: Some(self.clone()),
        })))
    }

    pub fn define(&self, name: impl Into<Rc<str>>, value: Value) {
        self.0.borrow_mut().vars.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let mut current = Some(self.clone());
        while let Some(env) = current {
            let scope = env.0.borrow();
            if let Some(v) = scope.vars.get(name) {
                return Some(v.clone());
            }
            current = scope.parent.clone();
        }
        None
    }

    pub fn assign(&self, name: &str, value: Value) -> bool {
        let mut current = Some(self.clone());
        while let Some(env) = current {
            let mut scope = env.0.borrow_mut();
            if let Some(slot) = scope.vars.get_mut(name) {
                *slot = value;
                return true;
            }
            current = scope.parent.clone();
        }
        false
    }

    pub fn has_local(&self, name: &str) -> bool {
        self.0.borrow().vars.contains_key(name)
    }

    pub fn local_bindings(&self) -> Vec<(Rc<str>, Value)> {
        self.0
            .borrow()
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn visible_names(&self) -> Vec<Rc<str>> {
        let mut names = Vec::new();
        let mut current = Some(self.clone());
        while let Some(env) = current {
            let scope = env.0.borrow();
            names.extend(scope.vars.keys().cloned());
            current = scope.parent.clone();
        }
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups_fall_through_and_assignments_update_the_owner() {
        let global = Env::new();
        global.define("x", Value::from(1.0));
        let inner = global.child();
        assert_eq!(inner.get("x"), Some(Value::from(1.0)));

        inner.define("x", Value::from(2.0));
        assert_eq!(inner.get("x"), Some(Value::from(2.0)));
        assert_eq!(global.get("x"), Some(Value::from(1.0)));

        let deeper = inner.child();
        assert!(deeper.assign("x", Value::from(3.0)));
        assert_eq!(inner.get("x"), Some(Value::from(3.0)));
        assert_eq!(global.get("x"), Some(Value::from(1.0)));
        assert!(!deeper.assign("nope", Value::Null));
    }
}
