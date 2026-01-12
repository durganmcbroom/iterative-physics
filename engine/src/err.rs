use std::fmt::{write, Display, Formatter};

pub type EngineResult<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    // Math
    UnsatisfiedVariable(String),
    UnsatisfiedFunction(String),

    WrongNumberOfArguments {
        name: String, expected: usize, found: usize,
    },
    UnexpectedComparison,
    ExpectedComparison,
    RootFindingDepthExceeded,
    InvalidDimensions,
    InvalidToken(String),
    InvalidMathSyntax(&'static str)
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::UnsatisfiedVariable(x) => {
                write!(f, "Unsatisfied variable: {}. Make sure you properly define all required variables for this computation", x)
            }
            ErrorKind::UnsatisfiedFunction(x) => {
                write!(f, "Unsatisfied function: {}. This function is unknown, try defining it (eg. f(x)=5x)", x)
                
            }
            ErrorKind::WrongNumberOfArguments { 
                name, expected, found
            } => {
                write!(f, "Function {} takes {} arguments, but you only provided {}.", name, found, expected)
            }
            ErrorKind::UnexpectedComparison => {
                write!(f, "Unexpected equals sign!")
            }
            ErrorKind::ExpectedComparison => {
                write!(f, "Expected an equals sign!")
            }
            ErrorKind::RootFindingDepthExceeded => {
                write!(f, "Math too complicated, failed to find roots of function fast enough.")
            }
            ErrorKind::InvalidDimensions => {
                write!(f, "This matrix is the wrong size.")
            }
            ErrorKind::InvalidToken(x) => {
                write!(f, "Invalid token '{}', in your equation.", x)
            }
            ErrorKind::InvalidMathSyntax(x) => {
                write!(f, "Invalid math syntax: {}", x)
            }
        }
    }
}