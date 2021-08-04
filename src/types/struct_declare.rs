use std::{fmt::Display, rc::Rc};

use crate::types::DynType;

use super::{exception::Exception, list::List, value::Value};

#[derive(Debug)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<String>, // field name and its index
}

impl Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("(record ");

        string.push_str(self.name.as_str());
        string.push_str(" (");

        let mut fields = vec![];
        for field_name in &self.fields {
            fields.push(field_name.clone());
        }
        string.push_str(fields.join(" ").as_str());
        string.push_str("))");

        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub struct_type: Rc<StructType>,
    pub data: Value,
}

impl Struct {
    pub fn new(struct_type: Rc<StructType>, data: Value) -> Result<Self, Exception> {
        let mut list = List::new(data.clone());

        for _ in &*struct_type.fields {
            list.next().to_middle()?;
        }
        list.next().to_end()?;
        Ok(Self { struct_type, data })
    }

    pub fn get_field(self, required_field: String) -> Result<Value, Exception> {
        let mut fields = List::new(self.data);

        for name in &(&*self.struct_type).fields {
            let item = fields.next().to_middle()?;
            if name.eq(&required_field) {
                return Ok(item);
            }
        }

        Err(Exception {
            thrown_object: Value::new(
                DynType::Str(format!("{} field is not found", required_field)),
                None,
            ),
            traceback: vec![],
            previous_exception: None,
        })
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = List::new(self.data.clone());

        let mut string = String::from('(');
        string.push_str(self.struct_type.name.as_str());

        string.push_str(" (");
        for field_name in &self.struct_type.fields {
            let value = list.next().to_middle();
            if let Ok(v) = value {
                string.push_str(format!("({} {}) ", field_name, v.content).as_str());
            } else {
                string.push_str(format!("({} ??) ", field_name).as_str())
            }
        }
        string.push_str("))");
        write!(f, "{}", string)
    }
}
