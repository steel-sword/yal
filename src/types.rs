use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub type Value = Rc<DynType>;

pub fn value(value: DynType) -> Value {
    Rc::new(value)
}

#[derive(Debug, Clone)]
pub struct DotPair {
    pub left: Value,
    pub right: Value,
}

impl DotPair {
    pub fn to_value(self) -> Value {
        value(DynType::Pair(self))
    }
}

pub enum DynType {
    Nil,
    Number(f64),
    Str(String),
    Symbol(String),
    Quoted(Value),
    Pair(DotPair),
    Closure(Rc<dyn Fn(Value) -> Result<Value, String>>),
}

impl DynType {
    pub fn to_number(&self) -> Result<f64, String> {
        if let DynType::Number(num) = self {
            Ok(*num)
        } else {
            Err(format!("Expected Number, given, {}", &*self))
        }
    }

    pub fn to_symbol(&self) -> Result<String, String> {
        if let DynType::Symbol(string) = self {
            Ok(string.clone())
        } else {
            Err(format!("Expected Number, given, {}", &*self))
        }
    }

    pub fn to_pair(&self) -> Result<&DotPair, String> {
        if let DynType::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(format!("Expected Pair, given, {}", &*self))
        }
    }
}

impl Debug for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynType::Nil => write!(f, "nil"),
            DynType::Number(number) => write!(f, "Number({})", number),
            DynType::Str(string) => write!(f, "Str(\"{}\")", string),
            DynType::Symbol(symbol) => write!(f, "Symbol({})", symbol),
            DynType::Quoted(value) => write!(f, "Quoted({})", value),
            DynType::Pair(pair) => f
                .debug_struct("Pair")
                .field("left", &pair.left)
                .field("right", &pair.right)
                .finish(),
            DynType::Closure(_) => write!(f, "<Closure>"),
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
            Self::Quoted(quoted) => format!("'{}", quoted),
            Self::Pair(pair) => {
                let mut buffer = String::new();
                buffer.push_str(format!("({}", pair.left).as_str());
                let mut list = List::new(pair.right.clone());
                while let ListItem::Middle(value) = list.next() {
                    buffer.push_str(format!(" {}", value).as_str());
                }
                match list.next() {
                    ListItem::Last(v) => buffer.push_str(format!(" . {})", v).as_str()) ,
                    ListItem::End => buffer.push(')'),
                    ListItem::Middle(_) => unreachable!(),
                }
                buffer
            },
            Self::Closure(_) => String::from("<Closure>"),
        };
        write!(f, "{}", string)
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DynType::Number(num1), DynType::Number(num2)) => *num1 == *num2,
            (DynType::Str(string1), DynType::Str(string2)) => *string1 == *string2,
            (DynType::Quoted(value1), DynType::Quoted(value2)) => value1 == value2,
            (DynType::Pair(pair1), DynType::Pair(pair2)) => {
                pair1.left == pair2.left && pair1.right == pair2.right
            }

            (DynType::Nil, DynType::Nil) => true,
            (DynType::Symbol(_), DynType::Symbol(_)) => false,
            (DynType::Closure(_), DynType::Closure(_)) => false,
            _ => false,
        }
    }
}

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
                value
            )),
            ListItem::End => Err(format!("Unexpected end of list")),
        }
    }

    pub fn to_last(self) -> Result<Value, String> {
        match self {
            ListItem::Last(value) => Ok(value),
            ListItem::Middle(value) => Err(format!(
                "unexpected part of list. Most be last value of list, found {}",
                value
            )),
            ListItem::End => Err(format!("Unexpected end of list")),
        }
    }

    pub fn to_end(self) -> Result<(), String> {
        match self {
            ListItem::End => Ok(()),
            ListItem::Middle(value) => Err(format!("Expected end of list, found {}", value)),
            ListItem::Last(value) => Err(format!("Expected end of list, found {}", value)),
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
        match &*current_value {
            DynType::Nil => ListItem::End,
            DynType::Pair(dot_pair) => {
                self.current_value = dot_pair.right.clone();
                ListItem::Middle(dot_pair.left.clone())
            }
            _ => ListItem::Last(current_value),
        }
    }
}
