const ALLOWED_SYMBOL_BEGIN: &str =
    "abcdefghijklmnopqrstuvwxyzABCEDFGHIJKLMNOPQRSTUVWXYZ_+-*/!@$%^&*<>?=:";

const ALLOWED_SYNTAX_SYMBOLS: &str = "()";

#[derive(Debug, Clone)]
pub enum Token {
    Space,
    Dot,
    Quote,
    OpenBracket,
    CloseBracket,
    Number(f64),
    Str(String),
    Symbol(String),
}

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub line: u32,
    pub line_char: u16,
    pub token: Token,
}

pub type Text<'a> = dyn Iterator<Item = char> + 'a;

struct Context<'a> {
    line: u32,
    line_char: u16,
    text: &'a mut Text<'a>,
    current_char: char,
    next_char: Option<char>,
}

impl<'a> Context<'a> {
    fn new(text: &'a mut Text) -> Context<'a> {
        let next_char = text.next();
        Context::<'a> {
            line: 1,
            line_char: 0,
            text,
            current_char: ' ',
            next_char,
        }
    }
    fn next(&mut self) -> Option<char> {
        let temp_next_char = self.text.next();
        self.current_char = self.next_char?;
        self.next_char = temp_next_char;

        if self.current_char == '\n' {
            self.line += 1;
            self.line_char = 0;
        } else {
            self.line_char += 1;
        }

        self.next_char
    }

    fn position(&self) -> (u32, u16) {
        (self.line, self.line_char)
    }
}

fn read_str(context: &mut Context) -> Result<Lexeme, String> {
    let mut buffer = String::new();
    let (line, line_char) = context.position();

    while context.next().is_some() {
        if context.current_char == '"' {
            return Ok(Lexeme {
                line,
                line_char,
                token: Token::Str(buffer),
            });
        }
        buffer.push(context.current_char);
    }
    Err(String::from("end of the string is not found"))
}

fn read_number(context: &mut Context) -> Result<Lexeme, String> {
    let mut buffer = String::from(context.current_char);
    let (line, line_char) = context.position();
    let mut is_float = false;

    'it: while {
        if context
            .next_char
            .filter(|n| n.is_ascii_whitespace() || ALLOWED_SYNTAX_SYMBOLS.contains(*n))
            .is_some()
        {
            break 'it;
        }
        context.next();
        if context.current_char.is_ascii_digit() {
            buffer.push(context.current_char);
        } else if context.current_char == '.' && !is_float {
            is_float = true;
            buffer.push(context.current_char);
        } else if (context.current_char == '.' && is_float)
            || ALLOWED_SYMBOL_BEGIN.contains(context.current_char)
        {
            let (line, line_char) = context.position();
            return Err(format!(
                "Unexpected symbol '{}' at the {}-{}",
                context.current_char, line, line_char
            ));
        }
        context.next_char.is_some()
    } {}

    match buffer.parse() {
        Ok(n) => Ok(Lexeme {
            line,
            line_char,
            token: Token::Number(n),
        }),
        Err(_) => Err(format!(
            "Parse error at {}-{}, \"{}\" - content",
            line, line_char, buffer
        )),
    }
}

fn read_symbol(context: &mut Context) -> Result<Lexeme, String> {
    let mut buffer = String::from(context.current_char);
    let (line, line_char) = context.position();

    'it: while {
        if context
            .next_char
            .filter(|n| n.is_ascii_whitespace() || ALLOWED_SYNTAX_SYMBOLS.contains(*n))
            .is_some()
        {
            break 'it;
        }
        context.next();
        if ALLOWED_SYMBOL_BEGIN.contains(context.current_char)
            || context.current_char.is_ascii_digit()
        {
            buffer.push(context.current_char);
        } else {
            let (line, line_char) = context.position();
            return Err(format!(
                "Unexpected symbol '{}' at {}-{}",
                context.current_char, line, line_char
            ));
        }
        context.next_char.is_some()
    } {}
    Ok(Lexeme {
        line,
        line_char,
        token: Token::Symbol(buffer),
    })
}

fn skip_comment(context: &mut Context) -> Result<Lexeme, String> {
    let (line, line_char) = context.position();
    while let Some(ch) = context.next() {
        if ch == '\n' {
            break;
        }
    }
    Ok(Lexeme {
        line,
        line_char,
        token: Token::Space,
    })
}

fn work_with_char(context: &mut Context) -> Result<Lexeme, String> {
    let (line, line_char) = context.position();
    match context.current_char {
        '#' => skip_comment(context),
        '.' => {
            if context.next_char.unwrap_or(' ').is_ascii_whitespace() {
                Ok(Lexeme {
                    line,
                    line_char,
                    token: Token::Dot,
                })
            } else {
                return Err(format!(
                    "Unexpected symbol '{}' at {}-{}",
                    context.next_char.unwrap(),
                    line,
                    line_char
                ));
            }
        }
        '\'' => Ok(Lexeme {
            line,
            line_char,
            token: Token::Quote,
        }),
        '(' => Ok(Lexeme {
            line,
            line_char,
            token: Token::OpenBracket,
        }),
        ')' => Ok(Lexeme {
            line,
            line_char,
            token: Token::CloseBracket,
        }),
        '"' => read_str(context),
        n if n.is_ascii_digit() => read_number(context),
        s if ALLOWED_SYMBOL_BEGIN.contains(s) => read_symbol(context),
        p if p.is_ascii_whitespace() => Ok(Lexeme {
            line,
            line_char,
            token: Token::Space,
        }),
        x => {
            return Err(format!(
                "Unexpected symbol '{}' at {}-{}",
                x, line, line_char
            ))
        }
    }
}

pub fn lex(text: &mut Text) -> Result<Vec<Lexeme>, String> {
    let mut lexemes = vec![];
    let mut context = Context::new(text);

    while {
        context.next();
        let lexeme = work_with_char(&mut context)?;
        if let Token::Space = lexeme.token {
        } else {
            lexemes.push(lexeme);
        }
        context.next_char.is_some()
    } {}

    Ok(lexemes)
}
