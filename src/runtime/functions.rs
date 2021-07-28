use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    rc::Rc,
};

use crate::types::{value, DotPair, DynType, List, ListItem, Value};

fn lang_new(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let record_type = list.next().to_middle()?.to_struct_declare()?;
    let rest = list.current_value.clone();
    for _ in &*record_type.fields {
        list.next().to_middle()?;
    }
    list.next().to_end()?;

    Ok(value(DynType::Struct(record_type, rest)))
}

fn lang_apply(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let closure = list.next().to_middle()?.to_closure()?;
    let args = list.next().to_middle()?;
    list.next().to_end()?;
    closure(args)
}

fn lang_input(args: Value) -> Result<Value, String> {
    let mut buffer = String::new();
    List::new(args).next().to_end()?;
    match stdin().read_line(&mut buffer) {
        Ok(_) => Ok(value(DynType::Str(String::from(
            buffer.trim_end_matches('\n'),
        )))),
        Err(err) => Err(format!("Cannot read from stdio, cause: {}", err)),
    }
}

fn lang_println(arg: Value) -> Result<Value, String> {
    let mut list = List::new(arg);
    let printed = list.next().to_middle()?;
    list.next().to_end()?;
    println!("{}", printed);
    Ok(value(DynType::Nil))
}

fn lang_print(arg: Value) -> Result<Value, String> {
    let mut list = List::new(arg);
    let printed = list.next().to_middle()?;
    list.next().to_end()?;
    print!("{}", printed);
    stdout().flush().unwrap_or_else(|error| println!("Print error: {}", &error));
    Ok(value(DynType::Nil))
}

fn lang_num_add(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let mut accum = 0.0;

    while let ListItem::Middle(value) = list.next() {
        accum += value.to_number()?;
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(accum)))
}

fn lang_num_mul(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let mut accum = 1.0;

    while let ListItem::Middle(value) = list.next() {
        accum *= value.to_number()?;
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(accum)))
}

fn lang_num_sub(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let mut accum = list.next().to_middle()?.to_number()?;

    let next = list.next();
    if let ListItem::End = next {
        return Ok(value(DynType::Number(-accum)));
    }
    accum -= next.to_middle()?.to_number()?;
    while let ListItem::Middle(value) = list.next() {
        accum -= value.to_number()?;
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(accum)))
}

fn lang_num_div(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let mut accum = list.next().to_middle()?.to_number()?;

    while let ListItem::Middle(value) = list.next() {
        accum *= value.to_number()?;
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(accum)))
}

fn lang_equals(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let first = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        if first != current {
            return Ok(value(DynType::Nil));
        }
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(1.0)))
}

fn lang_not_equals(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let first = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        if first == current {
            return Ok(value(DynType::Nil));
        }
    }
    list.next().to_end()?;
    Ok(value(DynType::Number(1.0)))
}

fn lang_cons(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let left = list.next().to_middle()?;
    let right = list.next().to_middle()?;
    list.next().to_end()?;
    Ok(value(DynType::Pair(DotPair { left, right })))
}

fn lang_left(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let pair = list.next().to_middle()?;
    let pair = pair.to_pair()?;
    list.next().to_end()?;
    let left = pair.left.clone();
    Ok(left)
}

fn lang_right(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let pair = list.next().to_middle()?;
    let pair = pair.to_pair()?;
    list.next().to_end()?;
    let right = pair.right.clone();
    Ok(right)
}

fn lang_concat(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let mut accum = String::new();

    while let ListItem::Middle(value) = list.next() {
        accum += format!("{}", value).as_str();
    }
    list.next().to_end()?;
    Ok(value(DynType::Str(accum)))
}

fn lang_number(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let parameter = list.next().to_middle()?;
    list.next().to_end()?;

    let number = match &*parameter {
        DynType::Nil => value(DynType::Number(0.0)),
        DynType::Number(_) => parameter.clone(),
        DynType::Str(s) => value(DynType::Number(
            match s.parse::<f64>() {
                Ok(num) => num,
                Err(_) => return Err(format!("Cannot parse '{}' to int", s)),
            }
        )),
        other => return Err(format!("Cannot parse '{}' to int", other)),
    };

    Ok(number)
}

fn lang_str(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let item = list.next().to_middle()?;
    list.next().to_end()?;

    Ok(value(DynType::Str(format!("{}", item))))
}

fn lang_split(args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let text = list.next().to_middle()?.to_string();
    let pat = match list.next() {
        ListItem::Middle(s) => (&*s).to_string(),
        ListItem::Last(_) => return Err(format!("Syntax Error")),
        ListItem::End => " ".to_string(),
    };
    list.next().to_end()?;
    let mut splitted: Vec<_> = text.split(&pat).collect();
    splitted.reverse();

    let mut pair = value(DynType::Nil);
    for item in splitted {
        pair = value(DynType::Pair(DotPair {
            right: pair,
            left: value(DynType::Str(item.to_string()))
        }));
    }

    Ok(pair)
}


pub fn all_base_functions() -> HashMap<String, Value> {
    let mut functions = HashMap::new();

    functions.insert(
        "new".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_new(args))))
    );

    functions.insert(
        "apply".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_apply(args))))
    );

    functions.insert(
        "input".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_input(args)))),
    );
    functions.insert(
        "print".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_print(args)))),
    );
    functions.insert(
        "println".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_println(args)))),
    );
    functions.insert(
        "+".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_num_add(args)))),
    );
    functions.insert(
        "-".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_num_sub(args)))),
    );
    functions.insert(
        "*".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_num_mul(args)))),
    );
    functions.insert(
        "/".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_num_div(args)))),
    );
    functions.insert(
        "=".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_equals(args)))),
    );
    functions.insert(
        "!=".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_not_equals(args)))),
    );
    functions.insert(
        "cons".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_cons(args)))),
    );
    functions.insert(
        "left".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_left(args)))),
    );
    functions.insert(
        "right".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_right(args)))),
    );

    functions.insert(
        "concat".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_concat(args)))),
    );

    functions.insert(
        "number".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_number(args)))),
    );

    functions.insert(
        "str".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_str(args))))
    );

    functions.insert(
        "split".to_string(),
        value(DynType::Closure(Rc::new(|args| lang_split(args)))),
    );

    functions
}
