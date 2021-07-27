use crate::lexer::{Lexeme, Token};
use crate::types::{value, DotPair};
use crate::types::{DynType, Value};

fn is_next(lexeme: Option<Lexeme>) -> Result<Lexeme, String> {
    match lexeme {
        Some(v) => Ok(v),
        None => Err(format!("Unexpected end of file")),
    }
}

fn unexpected_token<T>(lexeme: &Lexeme) -> Result<T, String> {
    Err(format!(
        "Unexpected token {:?} at {}-{}",
        lexeme.token, lexeme.line, lexeme.line_char,
    ))
}

struct Parser<'a> {
    lexemes: &'a mut dyn Iterator<Item = Lexeme>,
    current_lexeme: Option<Lexeme>,
}

impl<'a> Parser<'a> {
    pub fn new(lexemes: &'a mut dyn Iterator<Item = Lexeme>) -> Parser<'a> {
        Parser::<'a> {
            lexemes,
            current_lexeme: None,
        }
    }

    fn next(&mut self) -> Option<Lexeme> {
        self.current_lexeme = self.lexemes.next();
        self.current_lexeme.clone()
    }

    fn parse_list(&mut self) -> Result<Value, String> {
        let left = match is_next(self.current_lexeme.clone())?.token {
            Token::CloseBracket => return Ok(value(DynType::Nil)),
            _ => self.parse_value()?,
        };

        let current_lexeme = is_next(self.next())?;
        let right = match current_lexeme.token {
            Token::Dot => {
                self.next();
                let result = self.parse_value()?;
                let lexeme = is_next(self.next())?;
                match &lexeme.token {
                    Token::CloseBracket => {}
                    _ => unexpected_token(&lexeme)?,
                }
                result
            }
            Token::CloseBracket => value(DynType::Nil),
            _ => self.parse_list()?,
        };
        Ok(value(DynType::Pair(DotPair { left, right })))
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        let current = match self.current_lexeme.clone() {
            Some(lexeme) => lexeme,
            None => return Err(format!("Unexpected end of file during value parsing")),
        };

        // expression can begin from
        Ok(value(match &current.token {
            Token::Quote => {
                self.next();
                DynType::Quoted(self.parse_value()?)
            }
            Token::OpenBracket => {
                self.next();
                return self.parse_list();
            }
            Token::Number(number) => DynType::Number(*number),
            Token::Str(string) => DynType::Str(string.clone()),
            Token::Symbol(symbol) => DynType::Symbol(symbol.clone()),

            // dots, close brackets, etc. are wrong begin tokens
            _ => unexpected_token(&current)?,
        }))
    }

    fn parse(&mut self) -> Result<Vec<Value>, String> {
        let mut lists = Vec::new();
        while let Some(lexeme) = self.next() {
            if let Token::OpenBracket = lexeme.token {
                self.next();
                lists.push(self.parse_list()?);
            } else {
                lists.push(self.parse_value()?);
            };
        }

        Ok(lists)
    }
}

pub fn parse(lexemes: &mut dyn Iterator<Item = Lexeme>) -> Result<Vec<Value>, String> {
    Parser::new(lexemes).parse()
}
