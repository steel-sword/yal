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


impl Display for DotPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(format!("({}", self.left).as_str());
        let mut list = List::new(self.right.clone());
        while let ListItem::Middle(value) = list.next() {
            buffer.push_str(format!(" {}", value).as_str());
        }
        match list.next() {
            ListItem::Last(v) => buffer.push_str(format!(" . {})", v).as_str()) ,
            ListItem::End => buffer.push(')'),
            ListItem::Middle(_) => unreachable!(),
        }
        
        write!(f, "{}", buffer)
    }
}



#[derive(Debug)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<String> // field name and its index
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
    pub fn new(struct_type: Rc<StructType>, data: Value) -> Result<Self, String> {
        let mut list = List::new(data.clone());

        for _ in &*struct_type.fields {
            list.next().to_middle()?;
        }
        list.next().to_end()?;
        Ok(Self { struct_type, data })
    }

    pub fn get_field(self, required_field: String) -> Result<Value, String> {
        let mut fields = List::new(self.data);

        for name in &(&*self.struct_type).fields {
            let item = fields.next().to_middle()?;
            if name.eq(&required_field)  {
                return Ok(item);
            }
        }

        Err("Nothing was found".to_string())
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
                string.push_str(format!("({} {}) ", field_name, v).as_str());
            } else {
                string.push_str(format!("({} ??) ", field_name).as_str())
            }
            string.push_str("))")
        }
        write!(f, "{}", string)
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
    StructDeclare(Rc<StructType>),
    Struct(Struct),
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

    pub fn to_struct(&self) -> Result<Struct, String> {
        if let DynType::Struct(value) = self {
            Ok(value.clone())
        } else {
            Err(format!("Expected Record, given, {}", &*self))
        }
    }

    pub fn to_closure(&self) -> Result<Rc<dyn Fn(Value) -> Result<Value, String>>, String> {
        if let DynType::Closure(closure) = self {
            Ok(closure.clone())
        } else {
            Err(format!("Expected Closure, given {}", self))
        }
    }

    pub fn to_struct_declare(&self) -> Result<Rc<StructType>, String> {
        if let DynType::StructDeclare(struct_type) = self {
            Ok(struct_type.clone())
        } else {
            Err(format!("Expected RecordDeclare, given, {}", &*self))
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
            DynType::Quoted(value) => write!(f, "Quoted({})", value),
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
                .finish()
            ,
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

impl PartialOrd for DynType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DynType::Number(x), DynType::Number(y)) => x.partial_cmp(y),
            (DynType::Str(x), DynType::Str(y)) => x.partial_cmp(y),
            _ => None
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
