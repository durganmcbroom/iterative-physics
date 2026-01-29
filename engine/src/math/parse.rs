use crate::err::{EngineResult, Error, ErrorKind};
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Operation {
    Add,
    Sub,
    Multi,
    Div,
    Exp,
}

#[derive(Debug, Clone)]
pub enum Node {
    Arithmetic {
        operation: Operation,
        left: Box<Node>,
        right: Box<Node>,
    },
    Number(f64),
    Variable(String),
    Function {
        name: String,
        args: Vec<Box<Node>>,
    },
    Comparison {
        left: Box<Node>,
        right: Box<Node>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Static
    Op(Operation),
    OpenParen,
    CloseParen,
    Comma,
    Equals,

    // Dynamic
    Number(f64),
    Text(String),
}

// Result, if ended, value
type Outcome<T> = EngineResult<Option<T>>;

macro_rules! accept {
    ($outcome:expr) => {
        if let Some(x) = $outcome? {
            x
        } else {
            return Ok(None);
        }
    };
}

macro_rules! success {
    ($outcome:expr) => {
        Ok(Some($outcome))
    };
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input.chars().peekable(),
            current: None,
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_character(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    fn exec(&mut self) -> Outcome<Token> {
        let mut character: char;

        loop {
            if let Some(char) = self.input.next() {
                character = char;
            } else {
                return Ok(None);
            }

            if character != ' ' {
                break;
            }
        }

        if Self::is_digit(character) {
            let mut number = Vec::new();

            loop {
                number.push(character);

                if let Some(char) = self.input.next_if(|x| Self::is_digit(*x) || *x == '.') {
                    character = char;
                } else {
                    break;
                }
            }

            let str = number.iter().map(|c| *c).collect::<String>();
            let float = str.parse::<f64>().map_err(|_x| {
                Error::new(ErrorKind::InvalidMathSyntax(
                    "Unable to convert number to float. ",
                ))
            })?;

            return Ok(Some(Token::Number(float)));
        }

        if Self::is_character(character) {
            let mut name = Vec::new();

            loop {
                name.push(character);

                if let Some(char) = self.input.next_if(|x| Self::is_character(*x) || *x == '_') {
                    character = char;
                } else {
                    break;
                }
            }

            // let slice = &self.input[index..index + len];
            let str = name.iter().map(|c| *c as char).collect::<String>();

            return Ok(Some(Token::Text(str)));
        }

        let token = match character {
            // Arithmetic operations
            '+' => Token::Op(Operation::Add),
            '-' => Token::Op(Operation::Sub),
            '*' => Token::Op(Operation::Multi),
            '/' => Token::Op(Operation::Div),
            '^' => Token::Op(Operation::Exp),
            // Parens
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            // Misc
            ',' => Token::Comma,
            '=' => Token::Equals,
            _ => return Err(Error::new(ErrorKind::InvalidToken(character.to_string()))),
        };

        Ok(Some(token))
    }

    pub fn lex(&mut self) -> Outcome<Token> {
        if self.current.is_none() {
            self.current = self.exec()?;
        }

        if let Some(c) = &self.current {
            Ok(Some(c.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn advance(&mut self) {
        self.current = None
    }

    pub fn next(&mut self) -> Outcome<Token> {
        let token = self.lex();
        if token.is_ok() {
            self.advance();
        }
        token
    }
}

pub fn parse(mut lexer: Lexer) -> EngineResult<Node> {
    if let Some(x) = expression(&mut lexer)? {
        Ok(x)
    } else {
        Err(Error::new(ErrorKind::InvalidMathSyntax(
            "Premature end of input",
        )))
    }
}

fn expression(lexer: &mut Lexer) -> Outcome<Node> {
    let left = accept!(addition(lexer));

    if let Some(Token::Equals) = lexer.next()? {
        return success!(Node::Comparison {
            left: Box::new(left),
            right: Box::new(accept!(addition(lexer))),
        });
    }

    success!(left)
}

fn addition(lexer: &mut Lexer) -> Outcome<Node> {
    let mut left = accept!(multiplication(lexer));

    let mut work = true;
    while work {
        work = false;

        if let Some(op) = lexer.lex()? {
            // Check if it is an operation
            if let Token::Op(op) = op {
                // Check if it is addition, or subtraction
                if matches!(op, Operation::Add | Operation::Sub) {
                    // Advance lexer
                    lexer.advance();

                    left = Node::Arithmetic {
                        operation: op,
                        left: Box::new(left),
                        right: Box::new(accept!(multiplication(lexer))),
                    };

                    work = true
                }
            }
        }
    }

    success!(left)
}

fn multiplication(lexer: &mut Lexer) -> Outcome<Node> {
    let mut left = accept!(signed(lexer));

    let mut work = true;
    while work {
        work = false;

        if let Some(op) = lexer.lex()? {
            // Check if it is an operation
            if let Token::Op(op) = op {
                // Check if it is multiplication, or division
                if matches!(op, Operation::Multi | Operation::Div) {
                    // Advance lexer
                    lexer.advance();
                    work = true;

                    left = Node::Arithmetic {
                        operation: op,
                        left: Box::new(left),
                        right: Box::new(accept!(signed(lexer))),
                    };
                }
            }
            if let Token::OpenParen = op {
                lexer.advance();
                work = true;

                left = Node::Arithmetic {
                    operation: Operation::Multi,
                    left: Box::new(left),
                    right: Box::new(accept!(addition(lexer))),
                };
                if lexer.lex()?.is_none() || accept!(lexer.next()) != Token::CloseParen {
                    return Err(Error::new(ErrorKind::InvalidMathSyntax(
                        "Expected close param",
                    )));
                }
            }
            if let Some(right) = exponentiation(lexer)? {
                work = true;

                left = Node::Arithmetic {
                    operation: Operation::Multi,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }
    }

    success!(left)
}

fn signed(lexer: &mut Lexer) -> Outcome<Node> {
    let token = accept!(lexer.lex());

    match token {
        Token::Op(Operation::Sub) => {
            lexer.advance();
            if let Some(op) = exponentiation(lexer)? {
                success!(Node::Arithmetic {
                    operation: Operation::Sub,
                    left: Box::new(Node::Number(0.0)),
                    right: Box::new(op),
                })
            } else {
                Err(Error::new(ErrorKind::InvalidMathSyntax(
                    "Expected expression after '-' sign.",
                )))
            }
        }
        _ => exponentiation(lexer),
    }
}

fn exponentiation(lexer: &mut Lexer) -> Outcome<Node> {
    let mut left = accept!(atom(lexer));

    let mut work = true;
    while work {
        work = false;

        if let Some(op) = lexer.lex()? {
            // Check if it is an operation
            if let Token::Op(op) = op {
                // Check if it is multiplication, or division
                if matches!(op, Operation::Exp) {
                    // Advance lexer
                    lexer.advance();

                    left = Node::Arithmetic {
                        operation: Operation::Exp,
                        left: Box::new(left),
                        right: Box::new(accept!(atom(lexer))),
                    };

                    work = true
                }
            }
        }
    }

    success!(left)
}

fn atom(lexer: &mut Lexer) -> Outcome<Node> {
    let atom = accept!(lexer.lex());

    match atom {
        Token::Number(x) => {
            lexer.advance();
            success!(Node::Number(x))
        }
        Token::OpenParen => {
            lexer.advance();
            let node = accept!(addition(lexer));
            if accept!(lexer.next()) != Token::CloseParen {
                return Err(Error::new(ErrorKind::InvalidMathSyntax("No close param")));
            }

            success!(node)
        }
        Token::Text(text) => {
            lexer.advance();

            let lex = lexer.lex()?;
            if lex.is_none() || lex.unwrap() != Token::OpenParen {
                return success!(Node::Variable(text));
            };

            lexer.advance();

            let mut args: Vec<Box<Node>> = Vec::new();

            let mut work = true;
            while work {
                work = false;
                args.push(Box::new(if let Some(param) = addition(lexer)? {
                        param
                    } else {
                        return Err(Error::new(ErrorKind::InvalidMathSyntax("Expecting another parameter (at least 1 parameter, and 1 value after ever comma is required).")));
                    }));

                if let Some(Token::Comma) = lexer.lex()? {
                    lexer.advance();
                    work = true;
                }
            }
            if let Some(Token::CloseParen) = lexer.next()? {
                success!(Node::Function { name: text, args })
            } else {
                Err(Error::new(ErrorKind::InvalidMathSyntax("No close param.")))
            }
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::parse::Lexer;

    #[test]
    fn test_lex() {
        let mut lex = Lexer::new("1+1+2f(x)^(3+3)");

        loop {
            if let Some(x) = lex.next().unwrap() {
                println!("Token: {:?}", x)
            } else {
                break;
            }
        }
    }

    #[test]
    fn test_parse() {
        let lex = Lexer::new("x = 5 + 5");

        let result = parse(lex).unwrap();
        println!("result: {:?}", result);
    }
}
