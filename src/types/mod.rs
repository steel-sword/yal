pub mod dot_pair;
pub mod exception;
pub mod list;
pub mod value;
pub mod struct_declare;

use std::{fmt::{Debug, Display}, rc::Rc,};

use self::{
    dot_pair::DotPair,
    exception::Exception,
    value::Value,
    struct_declare::{Struct, StructType},
};

pub enum DynType {
    Nil,
    Number(f64),
    Str(String),
    Symbol(String),
    Quoted(Value),
    Pair(DotPair),
    Closure(Rc<dyn Fn(Value) -> Result<Value, Exception>>),
    StructDeclare(Rc<StructType>),
    Struct(Struct),
}

impl DynType {
    pub fn to_number(&self) -> Result<f64, Exception> {
        if let DynType::Number(num) = self {
            Ok(*num)
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected Number, given, {}", &*self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn to_symbol(&self) -> Result<String, Exception> {
        if let DynType::Symbol(string) = self {
            Ok(string.clone())
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected Number, given, {}", &*self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn to_pair(&self) -> Result<&DotPair, Exception> {
        if let DynType::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected Pair, given, {}", &*self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn to_struct(&self) -> Result<Struct, Exception> {
        if let DynType::Struct(value) = self {
            Ok(value.clone())
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected Record, given, {}", &*self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn to_closure(&self) -> Result<Rc<dyn Fn(Value) -> Result<Value, Exception>>, Exception> {
        if let DynType::Closure(closure) = self {
            Ok(closure.clone())
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected Closure, given {}", self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }

    pub fn to_struct_declare(&self) -> Result<Rc<StructType>, Exception> {
        if let DynType::StructDeclare(struct_type) = self {
            Ok(struct_type.clone())
        } else {
            Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Expected RecordDeclare, given, {}", &*self)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    }
}

impl Debug for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DynType::Nil => write!(f, "nil"),
            DynType::Number(number) => write!(f, "Number({})", *number),
            DynType::Str(string) => write!(f, "Str(\"{}\")", string),
            DynType::Symbol(symbol) => write!(f, "Symbol({})", symbol),
            DynType::Quoted(value) => write!(f, "Quoted({})", value.content),
            DynType::Pair(pair) => f
                .debug_struct("Pair")
                .field("left", &pair.left)
                .field("right", &pair.right)
                .finish(),
            DynType::Closure(_) => write!(f, "<Closure>"),
            DynType::StructDeclare(struct_declare) => f
                .debug_struct("StructType")
                .field("name", &struct_declare.name)
                .field("fields", &struct_declare.fields)
                .finish(),
            &DynType::Struct(struct_value) => f
                .debug_struct("Struct")
                .field("struct_type", &struct_value.struct_type)
                .field("data", &struct_value.data)
                .finish(),
        }
    }
}

impl Display for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Nil => String::from("nil"),
            Self::Number(number) => number.to_string(),
            Self::Str(string) => string.clone(),
            Self::Symbol(symbol) => symbol.clone(),
            Self::Quoted(quoted) => format!("'{}", quoted.content),
            Self::Pair(pair) => pair.to_string(),
            Self::Closure(_) => String::from("<Closure>"),
            Self::StructDeclare(struct_declare) => struct_declare.to_string(),
            Self::Struct(struct_value) => struct_value.to_string(),
        };
        write!(f, "{}", string)
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DynType::Number(num1), DynType::Number(num2)) => *num1 == *num2,
            (DynType::Str(string1), DynType::Str(string2)) => *string1 == *string2,
            (DynType::Quoted(value1), DynType::Quoted(value2)) => value1.content == value2.content,
            (DynType::Pair(pair1), DynType::Pair(pair2)) => {
                pair1.left.content == pair2.left.content
                    && pair1.right.content == pair2.right.content
            }

            (DynType::Nil, DynType::Nil) => true,
            (DynType::Symbol(_), DynType::Symbol(_)) => false,
            (DynType::Closure(_), DynType::Closure(_)) => false,
            _ => false,
        }
    }
}

impl PartialOrd for DynType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DynType::Number(x), DynType::Number(y)) => x.partial_cmp(y),
            (DynType::Str(x), DynType::Str(y)) => x.partial_cmp(y),
            _ => None,
        }
    }
}
