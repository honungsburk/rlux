use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::LuxValue;



#[derive(Clone)]
struct EnvNode {
    vars: HashMap<String, LuxValue>,
    parent: Option<Rc<RefCell<EnvNode>>>,
}

impl EnvNode {
    pub fn new() -> Self {
        Self { vars: HashMap::new(), parent: None }
    }

    pub fn with_parent(parent: Rc<RefCell<EnvNode>>) -> Self {
        Self { vars: HashMap::new(), parent: Some(parent) }
    }
    
    pub fn define(&mut self, name: String, value: LuxValue) {
        self.vars.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: LuxValue) -> bool {
        if !self.vars.contains_key(&name) {
            return match self.parent {
                Some(ref p) => p.borrow_mut().assign(name, value),
                None => false,
            }
        }
        self.vars.insert(name, value);
        return true;
    }

    pub fn get(&self, name: &str) -> Option<LuxValue> {
        match self.vars.get(name) {
            Some(v) => Some(v.clone()),
            None => match &self.parent {
                Some(p) => p.borrow().get(name),
                None => None,
            }
        }
    }
}


impl fmt::Debug for EnvNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("EnvNode");
        // Print the `vars` map
        debug_struct.field("vars", &self.vars);

        // Print the `parent` if it exists
        if let Some(parent) = &self.parent {
            debug_struct.field("parent", &parent.borrow());
        } else {
            debug_struct.field("parent", &"None");
        }

        debug_struct.finish()
    }
}


#[derive(Clone)]
pub struct Environment {
    node: Rc<RefCell<EnvNode>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { node: Rc::new(RefCell::new(EnvNode::new())) }
    }


    pub fn extend(&self) -> Self {
        let node = Rc::new(RefCell::new(
            EnvNode::with_parent(self.node.clone())
        ));
        Environment { node }
    }

    pub fn pop(&self)  -> Option<Self> {
        self.node.borrow().parent.clone().map(|node| {
            Environment { node }
        })
    }

    pub fn assign(&mut self, name: String, value: LuxValue) -> bool {
        self.node.borrow_mut().assign(name, value)
    }
    
    pub fn assign_at(&mut self, name: String, value: LuxValue, depth: usize) -> bool {
        if let Some(mut ancestor) = self.ancestor(depth) {
            ancestor.assign(name, value)
        } else {
            false
        }
    }

    pub fn get(&self, name: &str) -> Option<LuxValue> {
        self.node.borrow().get(name)
    }

    pub fn define(&mut self, name: String, value: LuxValue) {
        self.node.borrow_mut().define(name, value);
    }

    pub fn get_at(&self, name: &str, depth: usize) -> Option<LuxValue> {
        let ancestor = self.ancestor(depth)?;
        ancestor.get(name)
    }

    fn ancestor(&self, depth: usize) -> Option<Environment> {
        let mut current = self.clone();
        for _ in 0..depth {
            current = current.pop()?;
        }
        Some(current)
    }

}


impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.node.borrow().fmt(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_define_and_get_variables() {
        let mut env = Environment::new();
        env.define("a".to_string(), LuxValue::String("a".to_string()));
        assert_eq!(env.get("a"), Some(LuxValue::String("a".to_string())));
    }

    #[test]
    fn test_can_assign_and_get_variables() {
        let mut env = Environment::new();
        env.define("a".to_string(), LuxValue::String("a".to_string()));
        env.assign("a".to_string(), LuxValue::String("b".to_string()));
        assert_eq!(env.get("a"), Some(LuxValue::String("b".to_string())));
    }

    #[test]
    fn test_can_get_variables_from_parent_env() {
        let mut env = Environment::new();
        env.define("a".to_string(), LuxValue::string("a"));
        let child = env.extend();
        assert_eq!(child.get("a"), Some(LuxValue::string("a")));
    }

    #[test]
    fn test_can_assign_to_parent_env() {
        let mut env = Environment::new();
        env.define("a".to_string(), LuxValue::string("a"));
        let mut child = env.extend();
        child.assign("a".to_string(), LuxValue::string("b"));
        assert_eq!(env.get("a"), Some(LuxValue::string("b")));
    }


    #[test]
    fn test_can_assign_at_depth() {
        let mut env = Environment::new();
        env.define("a".to_string(), LuxValue::string("a"));
        let mut child = env.extend();
        child.define("a".to_string(), LuxValue::string("b"));
        child.assign_at("a".to_string(), LuxValue::string("c"),1);


        assert_eq!(env.get("a"), Some(LuxValue::string("c")));
        assert_eq!(child.get("a"), Some(LuxValue::string("b")));
        assert_eq!(child.get_at("a", 1), Some(LuxValue::string("c")));
    }

}
