use std::fmt::Display;

use crate::types::list::{List, ListItem};

use super::value::Value;

#[derive(Debug, Clone)]
pub struct DotPair {
    pub left: Value,
    pub right: Value,
}


impl Display for DotPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(format!("({}", self.left.content).as_str());
        let mut list = List::new(self.right.clone());
        while let ListItem::Middle(value) = list.next() {
            buffer.push_str(format!(" {}", value.content).as_str());
        }
        match list.next() {
            ListItem::Last(v) => buffer.push_str(format!(" . {})", v.content).as_str()) ,
            ListItem::End => buffer.push(')'),
            ListItem::Middle(_) => unreachable!(),
        }
        
        write!(f, "{}", buffer)
    }
}

