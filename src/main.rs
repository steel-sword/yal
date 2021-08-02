use std::{
    env,
    io::{stdin, Read},
};

mod lexer;
mod parser;
mod runtime;
mod types;

fn lexemes(text: String) {
    match lexer::lex(&mut text.chars()) {
        Ok(lx) => lx
            .iter()
            .enumerate()
            .for_each(|(i, l)| println!("{}:\n{:#?}", i, l)),
        Err(err) => eprintln!("{}", err),
    }
}

fn tree(text: String) {
    match lexer::lex(&mut text.chars()) {
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

fn exec(text: String) {
    match lexer::lex(&mut text.chars()) {
        Ok(lexemes) => match parser::parse(&mut lexemes.into_iter()) {
            Ok(values) => match runtime::execute(&mut values.into_iter()) {
                Ok(_) => {}
                Err(err) => {
                    let lines: Vec<_> = text.lines().collect();
                    eprintln!("Traceback:");
                    for position in err.traceback.iter() {
                        if let Some(pos) = position {
                            eprintln!("{}-{}", pos.0, pos.1);
                            eprintln!("{}", lines[pos.0 as usize - 1]);
                            let mut arrow = std::iter::repeat("-").take(pos.1 as usize - 1).collect::<String>();
                            arrow.push('^');
                            eprintln!("{}", arrow);
                        }
                    }
                    eprintln!("Exception: {:#?}", &err.thrown_object.content);
                },
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
            println!("Ctrl^D");
            lexemes(buffer)
        }
        "--tree" => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            println!("Ctrl^D");
            tree(buffer)
        }
        "--exec" => {
            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer).unwrap();
            println!("Ctrl^D");
            exec(buffer)
        }
        m => eprintln!("{} is unknown mode", m),
    }
}
