use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use crate::types::{DynType, exception::Exception, value::Value};

pub struct Scope {
    pub variables: HashMap<String, Value>,
    pub outer_scope: Option<ScopeRef>,
}

pub type ScopeRef = Rc<RefCell<Scope>>;

impl Scope {
    pub fn new(outer_scope: Option<ScopeRef>) -> Scope {
        Scope {
            variables: HashMap::new(),
            outer_scope,
        }
    }

    pub fn variable(&self, name: &String) -> Result<Value, Exception> {
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else if let Some(outer_scope) = &self.outer_scope {
            outer_scope.borrow_mut().variable(name)
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("variable {} is undefined", name)), None
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn define_variable(&mut self, name: String, value: Value) -> Result<(), Exception> {
        if self.variables.contains_key(&name) {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("variable {} already exists", &name)), None
                ),
                traceback: vec![],
                previous_exception: None,
            })
        } else {
            self.variables.insert(name, value);
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum ScopeState {
    Global,
    Local,
    Expression,
}

impl ScopeState {
    fn order_value(&self) -> u8 {
        match self {
            ScopeState::Global => 1,
            ScopeState::Local => 2,
            ScopeState::Expression => 3,
        }
    }
}

impl PartialOrd for ScopeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.order_value().cmp(&other.order_value()))
    }
}

impl PartialEq for ScopeState {
    fn eq(&self, other: &Self) -> bool {
        self.order_value() == other.order_value()
    }
}
