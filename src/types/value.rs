use std::{fmt::Debug, rc::Rc};

use super::DynType;

#[derive(Clone)]
pub struct Value {
    pub content: Rc<DynType>,
    pub position: Option<(u32, u16)>,
}

impl Value {
    pub fn new(content: DynType, position: Option<(u32, u16)>) -> Self {
        Self {
            content: Rc::new(content),
            position,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.content, f)?;
        if let Some(position) = self.position {
            write!(f, " [{}, {}]", position.0, position.1)
        } else {
            write!(f, " []")
        }
    }
}
