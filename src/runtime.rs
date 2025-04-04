use crate::{ast::AsonValue, environment::Environment};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    NotEnoughArgument { given: u16, expected: u16 },
    TooMuchArgument { given: u16, expected: u16 },
    UndefinedSymbol,
    NotAFunction
}

pub type Callback = fn(&[AsonValue], &mut Environment) -> AsonValue;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum AsonExpectedArgs {
    AtLeast(u16),
    Exact(u16),
    None
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct AsonFunction {
    pub fun: Callback,
    pub expected_args: AsonExpectedArgs,
}

impl AsonFunction {
    pub fn new(fun: Callback, expected_args: AsonExpectedArgs) -> Self {
        Self {
            fun,
            expected_args,
        }
    }

    pub fn call(&self, args: &[AsonValue], env: &mut Environment) -> Result<AsonValue, RuntimeError> {
        match self.expected_args {
            AsonExpectedArgs::AtLeast(n) => {
                if n > args.len() as u16 {
                    return Err(RuntimeError::NotEnoughArgument { given: args.len() as u16, expected: n });
                }
            }
            AsonExpectedArgs::Exact(n) => {
                if n < args.len() as u16 {
                    return Err(RuntimeError::TooMuchArgument { given: args.len() as u16, expected: n });
                }
                if n > args.len() as u16 {
                    return Err(RuntimeError::NotEnoughArgument { given: args.len() as u16, expected: n });
                }
            }
            AsonExpectedArgs::None => if args.len() > 0 {
                return Err(RuntimeError::TooMuchArgument { given: args.len() as u16, expected: 0 });
            }
        }

        Ok((self.fun)(args, env))
    }
}
