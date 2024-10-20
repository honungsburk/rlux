use std::collections::HashMap;
use crate::expr_eval::Value;



pub struct Environment {
    vars: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) -> bool {
        if !self.vars.contains_key(&name) {
            return false;
        }
        self.vars.insert(name, value);
        return true;
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }
}
