use super::{DynType, exception::Exception, value::Value};

pub enum ListItem {
    Middle(Value),
    Last(Value),
    End,
}

impl ListItem {
    pub fn to_middle(self) -> Result<Value, Exception> {
        match self {
            ListItem::Middle(value) => Ok(value),
            ListItem::Last(value) => Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!(
                        "Unexpected part of list. Most be Pair, found {}",
                        value.content
                    )),
                    None
                ),
                traceback: vec![],
                previous_exception: None
            }),
            ListItem::End => Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Unexpected end of list")),
                    None
                ),
                traceback: vec![],
                previous_exception: None
            }),
        }
    }

    pub fn to_end(self) -> Result<(), Exception> {
        match self {
            ListItem::End => Ok(()),
            ListItem::Middle(value) | ListItem::Last(value) => Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected end of list, found {}", value.content)),
                    None
                ),
                traceback: vec![],
                previous_exception: None
            }),
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
