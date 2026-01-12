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

    UnsatisfiedMotion(String), // The name of the body

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