use std::{
    env,
    io::{stdin, Read},
};

mod lexer;
mod parser;
mod runtime;
mod types;

fn lexemes(text: &mut dyn Iterator<Item = char>) {
    match lexer::lex(text) {
        Ok(lx) => lx
            .iter()
            .enumerate()
            .for_each(|(i, l)| println!("{}:\n{:#?}", i, l)),
        Err(err) => eprintln!("{}", err),
    }
}

fn tree(text: &mut dyn Iterator<Item = char>) {
    match lexer::lex(text) {
        Ok(lexemes) => match parser::parse(&mut lexemes.into_iter()) {
            Ok(values) => values
                .iter()
                .enumerate()
                .for_each(|(i, v)| println!("{}:\n{:#?}", i, v)),
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}

fn exec(text: &mut dyn Iterator<Item = char>) {
    match lexer::lex(text) {
        Ok(lexemes) => match parser::parse(&mut lexemes.into_iter()) {
            Ok(values) => match runtime::execute(&mut values.into_iter()) {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err),
            },
            Err(err) => eprintln!("{}", err),
        },
        Err(err) => eprintln!("{}", err),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("There most be 1 argument, given {}", args.len() - 1);
        return;
    }

    let mode = &args[1];
    match mode.as_str() {
        "--lexemes" => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            lexemes(&mut buffer.chars())
        }
        "--tree" => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            tree(&mut buffer.chars())
        }
        "--exec" => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            exec(&mut buffer.chars())
        }
        m => eprintln!("{} is unknown mode", m),
    }
}
