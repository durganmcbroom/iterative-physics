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
        HashMap::from([("pi".to_string(), PI), ("e".to_string(), E)])
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
                operation: _,
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
    pub fn push(&self, equation: &Equation) -> Frame<'_> {
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

    #[cfg(test)]
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
    Ignore,
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
                            }
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
        Node::Comparison { left:_, right:_ } => Err(Error::new(UnexpectedComparison)),
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
