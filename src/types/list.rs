use super::{DynType, value::Value};

pub enum ListItem {
    Middle(Value),
    Last(Value),
    End,
}

impl ListItem {
    pub fn to_middle(self) -> Result<Value, String> {
        match self {
            ListItem::Middle(value) => Ok(value),
            ListItem::Last(value) => Err(format!(
                "unexpected part of list. Most be pair, found {}",
                value.content
            )),
            ListItem::End => Err(format!("Unexpected end of list")),
        }
    }

    pub fn to_end(self) -> Result<(), String> {
        match self {
            ListItem::End => Ok(()),
            ListItem::Middle(value) => Err(format!("Expected end of list, found {}", value.content)),
            ListItem::Last(value) => Err(format!("Expected end of list, found {}", value.content)),
        }
    }
}

pub struct List {
    pub current_value: Value,
}

impl List {
    pub fn new(start_value: Value) -> List {
        List {
            current_value: start_value,
        }
    }

    pub fn next(&mut self) -> ListItem {
        let current_value = self.current_value.clone();
        match &*current_value.content {
            DynType::Nil => ListItem::End,
            DynType::Pair(dot_pair) => {
                self.current_value = dot_pair.right.clone();
                ListItem::Middle(dot_pair.left.clone())
            }
            _ => ListItem::Last(current_value),
        }
    }
}
