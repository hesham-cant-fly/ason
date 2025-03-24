use crate::ast::AsonValue;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    NotEnoughArgument,
    TooMuchArgument,
}

pub type Callback = fn(&[AsonValue]) -> AsonValue;

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

    pub fn call(&self, args: &[AsonValue]) -> Result<AsonValue, RuntimeError> {
        match self.expected_args {
            AsonExpectedArgs::AtLeast(n) => {
                if n > args.len() as u16 {
                    return Err(RuntimeError::NotEnoughArgument);
                }
            }
            AsonExpectedArgs::Exact(n) => {
                if n < args.len() as u16 {
                    return Err(RuntimeError::TooMuchArgument);
                }
                if n > args.len() as u16 {
                    return Err(RuntimeError::NotEnoughArgument);
                }
            }
            AsonExpectedArgs::None => if args.len() > 0 {
                return Err(RuntimeError::TooMuchArgument);
            }
        }

        Ok((self.fun)(args))
    }
}
