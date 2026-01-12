use crate::err::{Error, ErrorKind};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

// Rows x Columns
#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<const M: usize, const N: usize> {
    pub content: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn empty() -> Self {
        Matrix {
            content: [[0.0; N]; M],
        }
    }

    pub fn new(content: [[f64; N]; M]) -> Self {
        Matrix { content }
    }

    pub fn plus(&self, other: &Matrix<M, N>) -> Matrix<M, N> {
        let mut new = Matrix::<M, N>::empty();

        for i in 0..M {
            for j in 0..N {
                new.content[i][j] = self.content[i][j] + other.content[i][j];
            }
        }

        new
    }

    pub fn scale(&self, scalar: f64) -> Matrix<M, N> {
        let mut new = Matrix::<M, N>::empty();

        for i in 0..M {
            for j in 0..N {
                new.content[i][j] = scalar * self.content[i][j];
            }
        }

        new
    }

    pub fn multiply<const P: usize>(&self, other: &Matrix<N, P>) -> Matrix<M, P> {
        let mut new = Matrix::<M, P>::empty();

        for m in 0..M {
            for p in 0..P {
                let mut val = 0.0;

                for n in 0..N {
                    val += self.content[m][n] * other.content[n][p]
                }

                new.content[m][p] = val;
            }
        }

        new
    }
}

impl<const M: usize> Matrix<M, 1> {
    pub fn vector(content: [f64; M]) -> Self {
        let mut vec = Matrix::<M, 1>::empty();

        for (i, x) in content.into_iter().enumerate() {
            vec.content[i][0] = x
        }

        vec
    }
}

impl<const M: usize, const N: usize> Display for Matrix<M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Columns
        for m in 0..M {
            // Rows

            write!(
                f,
                "{}",
                match m {
                    0 => "⎡",
                    i if i + 1 == M => "⎣",
                    _ => "⎢",
                }
            )?;
            for n in 0..N {
                write!(f, "{}", self.content[m][n])?;
                if n + 1 < N {
                    write!(f, " ")?;
                }
            }

            write!(
                f,
                "{}",
                match m {
                    0 => "⎤",
                    i if i + 1 == M => "⎦",
                    _ => "⎢",
                }
            )?;
            writeln!(f)?;
        }

        Ok(())
    }
}

pub type Column<const M: usize> = Matrix<M, 1>;

pub trait Vector: Clone + Display {
    fn dof() -> usize;
    fn new(value: Vec<f64>) -> Result<Self, Error>;
    fn empty() -> Self;
    fn get(&self, i: usize) -> &f64;
    fn magnitude(&self) -> f64;
    fn unit(&self) -> Self;
    fn plus(&self, other: &Self) -> Self;
    fn scale(&self, scalar: f64) -> Self;
    fn dot(&self, other: &Self) -> f64;
}

impl<const M: usize> Vector for Column<M> {
    fn dof() -> usize {
        M
    }

    fn new(value: Vec<f64>) -> Result<Self, Error> {
        let content: Result<[f64; M], _> = value.try_into();
        if let Ok(content) = content {
            Ok(Matrix::vector(content))
        } else {
            Err(Error::new(ErrorKind::InvalidDimensions))
        }
    }

    fn empty() -> Self {
        Matrix::empty()
    }

    fn get(&self, i: usize) -> &f64 {
        &self.content[i][0]
    }

    fn magnitude(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..M {
            sum += self.get(i).powf(2.0);
        }

        sum.sqrt()
    }

    fn unit(&self) -> Self {
        let x = self.magnitude();

        if x == 0.0 {
            return Self::empty();
        }

        self.scale(1.0 / x)
    }

    fn plus(&self, other: &Self) -> Self {
        self.plus(other)
    }

    fn scale(&self, scalar: f64) -> Self {
        self.scale(scalar)
    }

    fn dot(&self, other: &Self) -> f64 {
        let mut sum = 0.0;
        for i in 0..M {
            sum += self.get(i) * other.get(i);
        }
        sum
    }
}

impl<const D: usize> TryFrom<Vec<f64>> for Matrix<D, 1> {
    type Error = Error;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        let content: Result<[f64; D], _> = value.try_into();
        if let Ok(content) = content {
            Ok(Matrix::vector(content))
        } else {
            Err(Error::new(ErrorKind::InvalidDimensions))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    id: u8,
    node: parse::Node,
    // Dependencies on variables
    dependencies: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);

        println!("{}", a);
    }

    #[test]
    fn test_matrix_scaling() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);

        let res = a.scale(2.0);
        println!("{}", res);
    }

    #[test]
    fn test_matrix_addition() {
        let a = Matrix::new([[1.1, 2.0], [3.0, 4.0]]);
        let b = Matrix::new([[1.0, 1.0], [1.0, 1.0]]);

        let res = a.plus(&b);
        println!("{}", res);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix::new([[5.0, 6.0], [7.0, 8.0]]);

        // Expected Result:
        // [5+14, 6+16  ]    =    [19, 22]
        // [15+28, 18+32]    =    [43, 50]

        let res = a.multiply(&b);
        println!("{}", res);
    }

    #[test]
    fn test_vector_multiplication() {
        let a = Matrix::new([[1.0, 2.0], [3.0, 4.0]]);
        let b = Matrix::vector([5.0, 6.0]);

        // Expected Result:
        // [17, 39]

        let res = a.multiply(&b);
        println!("{}", res);
    }
}

pub mod parse {
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
                let float = str.parse::<f64>().map_err(|x| {
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

                    if let Some(char) = self.input.next_if(|x| Self::is_character(*x) || *x == '_')
                    {
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
}

pub mod solve {
    use crate::err;
    use crate::err::ErrorKind::{ExpectedComparison, UnexpectedComparison};
    use crate::err::{Error, ErrorKind};
    use crate::math::Equation;
    use crate::math::parse::{Lexer, Node, Operation, parse};
    use err::EngineResult;
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub enum Function {
        Mathematical {
            node: Node,
            arg_names: Vec<String>,
        },
        Baked {
            call_site: fn(Vec<f64>) -> f64,
            expected: usize,
        },
    }

    pub mod builtin {
        use crate::math::solve::Function;
        use std::collections::HashMap;
        use std::f64::consts::E;
        use std::f64::consts::PI;

        macro_rules! trig {
            ($name:ident, $func:ident) => {
                pub static $name: Function = Function::Baked {
                    call_site: |x| x[0].$func(),
                    expected: 1,
                };
            };
        }

        trig!(SIN, sin);
        trig!(ASIN, asin);
        trig!(COS, cos);
        trig!(ACOS, acos);
        trig!(TAN, tan);
        trig!(ATAN, atan);

        pub static LOG: Function = Function::Baked {
            call_site: |x| x[0].log(x[1]),
            expected: 2,
        };

        macro_rules! log {
            ($name:ident, $base:expr) => {
                pub static $name: Function = Function::Baked {
                    call_site: |x| x[0].log($base),
                    expected: 1,
                };
            };
        }

        log!(LN, E);
        log!(LOG10, 10.0);
        log!(LOG2, 2.0);

        pub static SQRT: Function = Function::Baked {
            call_site: |x| x[0].sqrt(),
            expected: 1,
        };
        pub static NRT: Function = Function::Baked {
            call_site: |x| x[0].powf(1.0 / x[1]),
            expected: 2,
        };

        pub fn functions() -> HashMap<String, Function> {
            HashMap::from([
                ("sin".to_string(), SIN.clone()),
                ("asin".to_string(), ASIN.clone()),
                ("cos".to_string(), COS.clone()),
                ("acos".to_string(), ACOS.clone()),
                ("tan".to_string(), TAN.clone()),
                ("atan".to_string(), ATAN.clone()),
                ("log".to_string(), LOG.clone()),
                ("ln".to_string(), LN.clone()),
                ("sqrt".to_string(), SQRT.clone()),
                ("nrt".to_string(), NRT.clone()),
            ])
        }

        pub fn constants() -> HashMap<String, f64> {
            HashMap::from([
                ("pi".to_string(), PI),
                ("e".to_string(), E)
            ])
        }
    }

    #[derive(Debug)]
    pub struct Environment {
        equations: Vec<Equation>,
        functions: HashMap<String, Function>,
        constants: HashMap<String, f64>,
    }

    impl Environment {
        pub fn new(
            equations: Vec<Equation>,
            functions: HashMap<String, Function>,
            constants: HashMap<String, f64>,
        ) -> Environment {
            Environment {
                equations,
                functions,
                constants,
            }
        }

        pub fn evaluate(&self, var: String, overrides: HashMap<String, f64>) -> EngineResult<f64> {
            evaluate(
                &Node::Variable(var),
                Frame {
                    environment: self,
                    stack: Default::default(),
                    memo: Rc::new(RefCell::new(overrides)),
                    local: Default::default(),
                },
            )
        }

        fn analyze(node: &Node, dependencies: &mut HashSet<String>) {
            match node {
                Node::Arithmetic {
                    operation,
                    left,
                    right,
                } => {
                    Self::analyze(left, dependencies);
                    Self::analyze(right, dependencies);
                }
                Node::Variable(name) => {
                    dependencies.insert(name.clone());
                }
                Node::Function { name, args } => {
                    dependencies.insert(name.clone());
                    for x in args {
                        Self::analyze(x, dependencies);
                    }
                }
                Node::Comparison { left, right } => {
                    Self::analyze(left, dependencies);
                    Self::analyze(right, dependencies);
                }
                _ => {}
            }
        }

        pub fn build(
            expressions: Vec<&str>,
            mut functions: HashMap<String, Function>,
            constants: HashMap<String, f64>,
        ) -> EngineResult<Environment> {
            let expressions = expressions
                .iter()
                .map(|t| parse(Lexer::new(t)))
                .collect::<EngineResult<Vec<_>>>()?;

            let mut equations = Vec::<Equation>::new();

            let mut id: u8 = 0;

            macro_rules! eq {
                ($node:expr) => {{
                    let mut dependencies = HashSet::<String>::new();

                    Self::analyze(&$node, &mut dependencies);
                    equations.push(Equation {
                        node: $node,
                        dependencies,
                        id: id,
                    });

                    id += 1;
                }};
            }

            for x in expressions {
                if let Node::Comparison { left, right } = x.clone() {
                    if let Node::Function { name, args } = *left {
                        let params = args
                            .iter()
                            .map(|it| {
                                if let Node::Variable(name) = it.as_ref() {
                                    Some(name.clone())
                                } else {
                                    None
                                }
                            })
                            .collect::<Option<Vec<_>>>();

                        if let Some(params) = params {
                            functions.insert(
                                name.clone(),
                                Function::Mathematical {
                                    node: *right.clone(),
                                    arg_names: params,
                                },
                            );
                        } else {
                            eq!(x)
                        }
                    } else {
                        eq!(x)
                    }
                }
            }

            Ok(Environment {
                equations,
                functions,
                constants,
            })
        }
    }

    #[derive(Clone)]
    pub struct Frame<'a> {
        environment: &'a Environment,
        stack: HashSet<u8>,
        // Memoized global variables
        memo: Rc<RefCell<HashMap<String, f64>>>,
        // Local variables
        local: HashMap<String, f64>,
    }

    impl<'a> Frame<'a> {
        pub fn push(&self, equation: &Equation) -> Frame {
            let mut clone = self.clone();
            clone.stack.insert(equation.id);
            clone.clear_locals();
            clone
        }

        pub fn visited(&self, equation: &Equation) -> bool {
            self.stack.contains(&equation.id)
        }

        pub fn memo<T: Into<String>>(&mut self, name: T, value: f64) {
            self.memo.borrow_mut().insert(name.into(), value);
        }

        pub fn local(&mut self, name: String, value: f64) {
            self.local.insert(name, value);
        }

        pub fn clear_locals(&mut self) {
            self.local.clear();
        }

        pub fn lookup(&self, name: &String) -> Option<f64> {
            self.local
                .get(name)
                .cloned()
                .or(self.memo.borrow().get(name).cloned())
        }

        pub(crate) fn empty(environment: &'a Environment) -> Frame<'a> {
            Frame {
                environment,
                stack: Default::default(),
                memo: Default::default(),
                local: Default::default(),
            }
        }
    }

    enum VariableResolution {
        Success(f64),
        UnsatisfiedVariable(String),
        Ignore
    }

    pub fn evaluate(node: &Node, frame: Frame) -> EngineResult<f64> {
        match node {
            Node::Arithmetic {
                operation,
                left,
                right,
            } => {
                let left = evaluate(left, frame.clone())?;
                let right = evaluate(right, frame.clone())?;

                let result = match operation {
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                    Operation::Multi => left * right,
                    Operation::Div => left / right,
                    Operation::Exp => left.powf(right),
                };

                Ok(result)
            }
            Node::Number(n) => Ok(*n),
            Node::Variable(name) => {
                if let Some(x) = frame.lookup(name) {
                    return Ok(x);
                }

                if let Some(x) = frame.environment.constants.get(name) {
                    return Ok(*x);
                }

                let ret = frame
                    .environment
                    .equations
                    .iter()
                    .filter(|x| x.dependencies.contains(name))
                    .map(|eq| {
                        // TODO Multivariate roots
                        if frame.visited(eq) {
                            return Ok(VariableResolution::Ignore);
                        }

                        let root_expr = if let Node::Comparison { left, right } = eq.node.clone() {
                            Node::Arithmetic {
                                operation: Operation::Sub,
                                right: right.clone(),
                                left: left.clone(),
                            }
                        } else {
                            return Err(Error::new(ExpectedComparison));
                        };

                        let root = find_root(&root_expr, name, 0.0, frame.push(eq));

                        // TODO better errors here
                        match root {
                            Ok(val) => Ok(VariableResolution::Success(val)),
                            Err(e) => match e.kind {
                                ErrorKind::UnsatisfiedVariable(x) => {
                                    Ok(VariableResolution::UnsatisfiedVariable(x))
                                },
                                _ => Err(e),
                            },
                        }
                    })
                    .collect::<EngineResult<Vec<VariableResolution>>>()?;
                    // .iter()
                    // .filter(|x| !matches!(x, VariableResolution::Ignore))
                    // .collect::<Vec<VariableResolution>>();
                let mut unsatisfied_variables = Vec::new();

                for x in ret {
                    match x {
                        VariableResolution::Success(x) => {
                            return Ok(x);
                        }
                        VariableResolution::UnsatisfiedVariable(name) => {
                            unsatisfied_variables.push(name);
                        }
                        VariableResolution::Ignore => {
                            // Ignore
                        }
                    }
                }

                if unsatisfied_variables.is_empty() {
                    Err(Error::new(ErrorKind::UnsatisfiedVariable(name.clone())))
                } else {
                    Err(Error::new(ErrorKind::UnsatisfiedVariable(
                        unsatisfied_variables.first().unwrap().clone(),
                    )))

                }
                //
                // if let Some(ret) = ret.iter().find(|x| matches!(x, VariableResolution::Success(_))) {
                //     Ok(ret)
                // } else {
                //     Err(Error::new(ErrorKind::UnsatisfiedVariable(name.clone())))
                // }
            }
            Node::Function { name, args } => {
                let f = frame
                    .environment
                    .functions
                    .get(name)
                    .ok_or_else(|| Error::new(ErrorKind::UnsatisfiedFunction(name.clone())))?;

                let args = args
                    .iter()
                    .map(|exp| evaluate(exp, frame.clone()))
                    .collect::<Result<Vec<f64>, Error>>()?;

                match f {
                    Function::Mathematical { node, arg_names } => {
                        if args.len() != arg_names.len() {
                            Err(Error::new(ErrorKind::WrongNumberOfArguments {
                                name: name.clone(),
                                expected: arg_names.len(),
                                found: args.len(),
                            }))
                        } else {
                            let mut frame = frame.clone();
                            frame.clear_locals();

                            for (x, name) in args.iter().zip(arg_names.iter()) {
                                frame.local(name.clone(), *x);
                            }

                            evaluate(node, frame)
                        }
                    }
                    Function::Baked {
                        call_site,
                        expected,
                    } => {
                        if args.len() != *expected {
                            Err(Error::new(ErrorKind::WrongNumberOfArguments {
                                name: name.clone(),
                                expected: *expected,
                                found: args.len(),
                            }))
                        } else {
                            Ok(call_site(args))
                        }
                    }
                }
            }
            Node::Comparison { left, right } => Err(Error::new(UnexpectedComparison)),
        }
    }

    ///
    /// Newton's method implementation of root finding
    ///
    fn find_root(node: &Node, target: &str, guess: f64, mut frame: Frame) -> EngineResult<f64> {
        const EPSILON: f64 = 0.00001;
        const MAX_DEPTH: usize = 10000;

        let mut last = guess;

        for _ in 0..MAX_DEPTH {
            frame.local(target.to_string(), last);
            let x_i = evaluate(node, frame.clone())?;

            frame.local(target.to_string(), last + EPSILON);
            let x_i_epsilon = evaluate(node, frame.clone())?;

            let slope = (x_i_epsilon - x_i) / (EPSILON);

            let next = last - x_i / slope;

            if (last - next).abs() < EPSILON {
                frame.memo(target.to_string(), next);

                return Ok(next);
            }

            last = next;
        }

        Err(Error::new(ErrorKind::RootFindingDepthExceeded))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_evaluate_roots() {
            let input = "x*(1+x^2)^(-1/2)";
            let bytes = input
                .as_bytes()
                .iter()
                .map(|x| *x as char)
                .collect::<Vec<_>>();
            let node = parse(Lexer::new(input)).unwrap();
            let env = Environment::build(vec![], HashMap::new(), HashMap::new()).unwrap();
            let frame = Frame::empty(&env);

            let root = find_root(&node, "x", -1.0, frame);

            println!("{:?}", root);
        }

        #[test]
        fn test_evaluate_multiple_equations() {
            let env = Environment::build(vec![""], HashMap::new(), HashMap::new()).unwrap();

            let mut frame = Frame::empty(&env);
            frame.memo("hati", 1.0);
            frame.memo("hatj", 0.0);

            let res = evaluate(&Node::Variable("a".to_string()), frame).unwrap();

            println!("res: {:?}", res);
        }
    }
}

pub mod integration {
    // Displacement, velocity
    type ParticleState = (f64, f64);

    pub fn leapfrog_displacement(
        delta: f64,
        displacement: f64,
        velocity: f64,
        acceleration: f64,
    ) -> f64 {
        let velocity = velocity * delta;
        let accel = 1.0 / 2.0 * acceleration * delta.powf(2.0);

        displacement + velocity + accel
    }

    pub fn leapfrog_velocity(
        delta: f64,
        velocity: f64,
        acceleration: f64,
        next_acceleration: f64,
    ) -> f64 {
        let accel = 1.0 / 2.0 * (acceleration + next_acceleration) * delta;

        velocity + accel
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_leapfrog() {
            let acceleration = 2.0;
            let mut velocity = 0.0;
            let mut displacement = 0.0;

            for _ in 0..100 {
                displacement = leapfrog_displacement(0.1, displacement, velocity, acceleration);
                velocity = leapfrog_velocity(0.1, velocity, acceleration, acceleration);

                println!("{}, {}", displacement, velocity);
            }
        }
    }
}
