use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    rc::Rc,
    vec,
};

use crate::types::{
    dot_pair::DotPair,
    exception::Exception,
    list::{List, ListItem},
    value::Value,
    DynType, Struct,
};

fn lang_new(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let struct_type = list.next().to_middle()?.content.to_struct_declare()?;
    let rest = list.current_value.clone();
    Ok(Value::new(
        DynType::Struct(Struct::new(struct_type, rest)?),
        None,
    ))
}

fn lang_apply(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let closure = list.next().to_middle()?.content.to_closure()?;
    let args = list.next().to_middle()?;
    list.next().to_end()?;
    closure(args)
}

fn lang_input(args: Value) -> Result<Value, Exception> {
    let mut buffer = String::new();
    List::new(args).next().to_end()?;
    match stdin().read_line(&mut buffer) {
        Ok(_) => Ok(Value::new(
            DynType::Str(String::from(buffer.trim_end_matches('\n'))),
            None,
        )),
        Err(err) => Err(Exception {
            thrown_object: Value::new(
                DynType::Str(format!("Cannot read from stdio, cause: {}", err)),
                None,
            ),
            traceback: vec![],
            previous_exception: None,
        }),
    }
}

fn lang_println(arg: Value) -> Result<Value, Exception> {
    let mut list = List::new(arg);

    while let ListItem::Middle(item) = list.next() {
        println!("{}", item.content);
    }
    list.next().to_end()?;

    Ok(Value::new(DynType::Nil, None))
}

fn lang_print(arg: Value) -> Result<Value, Exception> {
    let mut list = List::new(arg);
    
    while let ListItem::Middle(item) = list.next() {
        print!("{}", item.content);
    }
    list.next().to_end()?;

    stdout()
        .flush()
        .unwrap_or_else(|error| println!("Print error: {}", &error));
    Ok(Value::new(DynType::Nil, None))
}

fn lang_num_add(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut accum = 0.0;

    while let ListItem::Middle(value) = list.next() {
        accum += value.content.to_number()?;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(accum), None))
}

fn lang_num_mul(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut accum = 1.0;

    while let ListItem::Middle(value) = list.next() {
        accum *= value.content.to_number()?;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(accum), None))
}

fn lang_num_sub(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut accum = list.next().to_middle()?.content.to_number()?;

    let next = list.next();
    if let ListItem::End = next {
        return Ok(Value::new(DynType::Number(-accum), None));
    }
    accum -= next.to_middle()?.content.to_number()?;
    while let ListItem::Middle(value) = list.next() {
        accum -= value.content.to_number()?;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(accum), None))
}

fn lang_num_div(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut accum = list.next().to_middle()?.content.to_number()?;

    while let ListItem::Middle(value) = list.next() {
        accum /= value.content.to_number()?;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(accum), None))
}

fn lang_num_mod(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let x = list.next().to_middle()?.content.to_number()?;
    let y = list.next().to_middle()?.content.to_number()?;
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(x % y), None))
}

fn lang_equals(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let first = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        if first.content != current.content {
            return Ok(Value::new(DynType::Nil, None));
        }
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_not_equals(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let first = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        if first.content == current.content {
            return Ok(Value::new(DynType::Nil, None));
        }
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_greater_than(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut previous = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        match previous.content.partial_cmp(&current.content) {
            Some(cmp) => match cmp {
                std::cmp::Ordering::Less => return Ok(Value::new(DynType::Nil, None)),
                std::cmp::Ordering::Equal => return Ok(Value::new(DynType::Nil, None)),
                std::cmp::Ordering::Greater => {}
            },
            None => {
                return Err(Exception {
                    thrown_object: Value::new(
                        DynType::Str(format!(
                            "Uncomparable types {:?} and {:?}",
                            previous.content, current.content
                        )),
                        None,
                    ),
                    traceback: vec![],
                    previous_exception: None,
                })
            }
        }

        previous = current;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_greater_than_or_equals(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut previous = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        match previous.content.partial_cmp(&current.content) {
            Some(cmp) => match cmp {
                std::cmp::Ordering::Less => return Ok(Value::new(DynType::Nil, None)),
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {}
            },
            None => {
                return Err(Exception {
                    thrown_object: Value::new(
                        DynType::Str(format!(
                            "Uncomparable types {:?} and {:?}",
                            previous.content, current.content
                        )),
                        None,
                    ),
                    traceback: vec![],
                    previous_exception: None,
                })
            }
        }

        previous = current;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_less_than(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut previous = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        match previous.content.partial_cmp(&current.content) {
            Some(cmp) => match cmp {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => return Ok(Value::new(DynType::Nil, None)),
                std::cmp::Ordering::Greater => return Ok(Value::new(DynType::Nil, None)),
            },
            None => {
                return Err(Exception {
                    thrown_object: Value::new(
                        DynType::Str(format!(
                            "Uncomparable types {:?} and {:?}",
                            previous.content, current.content
                        )),
                        None,
                    ),
                    traceback: vec![],
                    previous_exception: None,
                })
            }
        }

        previous = current;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_less_than_or_equals(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut previous = list.next().to_middle()?;
    while let ListItem::Middle(current) = list.next() {
        match previous.content.partial_cmp(&current.content) {
            Some(cmp) => match cmp {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => return Ok(Value::new(DynType::Nil, None)),
            },
            None => {
                return Err(Exception {
                    thrown_object: Value::new(
                        DynType::Str(format!(
                            "Uncomparable types {:?} and {:?}",
                            previous.content, current.content
                        )),
                        None,
                    ),
                    traceback: vec![],
                    previous_exception: None,
                })
            }
        }

        previous = current;
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Number(1.0), None))
}

fn lang_cmp(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let first = list.next().to_middle()?;
    let second = list.next().to_middle()?;
    list.next().to_end()?;

    match first.content.partial_cmp(&second.content) {
        Some(cmp) => match cmp {
            std::cmp::Ordering::Less => Ok(Value::new(DynType::Number(-1.0), None)),
            std::cmp::Ordering::Equal => Ok(Value::new(DynType::Number(0.0), None)),
            std::cmp::Ordering::Greater => Ok(Value::new(DynType::Number(1.0), None)),
        },
        None => Err(Exception {
            thrown_object: Value::new(
                DynType::Str(format!(
                    "Uncomparable types {:?} and {:?}",
                    first.content, second.content
                )),
                None,
            ),
            traceback: vec![],
            previous_exception: None,
        }),
    }
}

fn lang_pair(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let left = list.next().to_middle()?;
    let right = list.next().to_middle()?;
    list.next().to_end()?;
    Ok(Value::new(DynType::Pair(DotPair { left, right }), None))
}

fn lang_left(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let pair = list.next().to_middle()?;
    let pair = pair.content.to_pair()?;
    list.next().to_end()?;
    let left = pair.left.clone();
    Ok(left)
}

fn lang_right(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let pair = list.next().to_middle()?;
    let pair = pair.content.to_pair()?;
    list.next().to_end()?;
    let right = pair.right.clone();
    Ok(right)
}

fn lang_concat(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let mut accum = String::new();

    while let ListItem::Middle(value) = list.next() {
        accum += format!("{}", value.content).as_str();
    }
    list.next().to_end()?;
    Ok(Value::new(DynType::Str(accum), None))
}

fn lang_number(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let parameter = list.next().to_middle()?;
    list.next().to_end()?;

    let number = match &*parameter.content {
        DynType::Nil => Value::new(DynType::Number(0.0), None),
        DynType::Number(_) => parameter.clone(),
        DynType::Str(s) => Value::new(
            DynType::Number(match s.parse::<f64>() {
                Ok(num) => num,
                Err(_) => {
                    return Err(Exception {
                        thrown_object: Value::new(
                            DynType::Str(format!("Cannot parse '{}' to int", s)),
                            None,
                        ),
                        traceback: vec![],
                        previous_exception: None,
                    })
                }
            }),
            None,
        ),
        other => {
            return Err(Exception {
                thrown_object: Value::new(
                    DynType::Str(format!("Cannot parse '{}' to int", other)),
                    None,
                ),
                traceback: vec![],
                previous_exception: None,
            })
        }
    };

    Ok(number)
}

fn lang_str(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let item = list.next().to_middle()?;
    list.next().to_end()?;

    Ok(Value::new(DynType::Str(format!("{}", item.content)), None))
}

fn lang_split(args: Value) -> Result<Value, Exception> {
    let mut list = List::new(args);
    let text = list.next().to_middle()?.content.to_string();

    let mut splitted: Vec<String> = match list.next() {
        ListItem::Middle(s) => text
            .split(&(&*s.content).to_string())
            .map(str::to_string)
            .collect(),

        ListItem::End => text.split_whitespace().map(str::to_string).collect(),

        ListItem::Last(_) => {
            return Err(Exception {
                thrown_object: Value::new(DynType::Str(format!("Syntax Error")), None),
                traceback: vec![],
                previous_exception: None,
            })
        }
    };
    list.next().to_end()?;

    splitted.reverse();
    let mut pair = Value::new(DynType::Nil, None);
    for item in splitted {
        pair = Value::new(
            DynType::Pair(DotPair {
                right: pair,
                left: Value::new(DynType::Str(item.to_string()), None),
            }),
            None,
        );
    }

    Ok(pair)
}

pub fn all_base_functions() -> HashMap<String, Value> {
    let mut functions = HashMap::new();

    functions.insert(
        "new".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_new(args))), None),
    );

    functions.insert(
        "apply".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_apply(args))), None),
    );

    functions.insert(
        "input".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_input(args))), None),
    );
    functions.insert(
        "print".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_print(args))), None),
    );
    functions.insert(
        "println".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_println(args))), None),
    );
    functions.insert(
        "+".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_num_add(args))), None),
    );
    functions.insert(
        "-".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_num_sub(args))), None),
    );
    functions.insert(
        "*".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_num_mul(args))), None),
    );
    functions.insert(
        "/".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_num_div(args))), None),
    );
    functions.insert(
        "%".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_num_mod(args))), None),
    );
    functions.insert(
        "=".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_equals(args))), None),
    );
    functions.insert(
        "!=".to_string(),
        Value::new(
            DynType::Closure(Rc::new(|args| lang_not_equals(args))),
            None,
        ),
    );
    functions.insert(
        ">".to_string(),
        Value::new(
            DynType::Closure(Rc::new(|args| lang_greater_than(args))),
            None,
        ),
    );
    functions.insert(
        ">=".to_string(),
        Value::new(
            DynType::Closure(Rc::new(|args| lang_greater_than_or_equals(args))),
            None,
        ),
    );
    functions.insert(
        "<".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_less_than(args))), None),
    );
    functions.insert(
        "<=".to_string(),
        Value::new(
            DynType::Closure(Rc::new(|args| lang_less_than_or_equals(args))),
            None,
        ),
    );
    functions.insert(
        "cmp".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_cmp(args))), None),
    );
    functions.insert(
        "pair".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_pair(args))), None),
    );
    functions.insert(
        "left".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_left(args))), None),
    );
    functions.insert(
        "right".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_right(args))), None),
    );
    functions.insert(
        "concat".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_concat(args))), None),
    );
    functions.insert(
        "number".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_number(args))), None),
    );
    functions.insert(
        "str".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_str(args))), None),
    );
    functions.insert(
        "split".to_string(),
        Value::new(DynType::Closure(Rc::new(|args| lang_split(args))), None),
    );
    functions
}
