use core::fmt;
use std::{collections::HashMap, ops};

use crate::{environment::Environment, lexer::Lexer, parser::Parser, token::TokenList};

#[allow(dead_code)]
#[derive(Debug)]
pub enum AsonExpr {
    Value(AsonValue),
    Symbol(String),
    ExprS(Vec<AsonExpr>, String),
    None,
}

impl AsonExpr {
    pub fn eval(&self, env: &mut Environment) -> AsonValue {
        match self {
            AsonExpr::Value(ason_value) => (*ason_value).clone(),
            AsonExpr::Symbol(_) => todo!(),
            AsonExpr::ExprS(vec, callee) => self.eval_expr_s(vec, &callee, env),
            AsonExpr::None => AsonValue::Null,
        }
    }

    fn eval_expr_s(&self, params: &Vec<AsonExpr>, callee: &String, env: &mut Environment) -> AsonValue {
        let args: Vec<AsonValue> = params
            .iter()
            .map(|e| e.eval(env))
            .collect();

        env.call_fn(&callee, args)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum AsonNumber {
    Integer(i64),
    Float(f64),
}

impl ops::Add for AsonNumber {
    type Output = AsonNumber;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            AsonNumber::Integer(i) => match rhs {
                AsonNumber::Integer(i2) => AsonNumber::Integer(i + i2),
                AsonNumber::Float(f) => AsonNumber::Float((i as f64) + f),
            },
            AsonNumber::Float(f) => match rhs {
                AsonNumber::Integer(i) => AsonNumber::Float(f + (i as f64)),
                AsonNumber::Float(f2) => AsonNumber::Float(f + f2),
            },
        }
    }
}

impl ops::Sub for AsonNumber {
    type Output = AsonNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            AsonNumber::Integer(i) => match rhs {
                AsonNumber::Integer(i2) => AsonNumber::Integer(i - i2),
                AsonNumber::Float(f) => AsonNumber::Float((i as f64) - f),
            },
            AsonNumber::Float(f) => match rhs {
                AsonNumber::Integer(i) => AsonNumber::Float(f - (i as f64)),
                AsonNumber::Float(f2) => AsonNumber::Float(f - f2),
            },
        }
    }
}

impl ops::Mul for AsonNumber {
    type Output = AsonNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            AsonNumber::Integer(i) => match rhs {
                AsonNumber::Integer(i2) => AsonNumber::Integer(i * i2),
                AsonNumber::Float(f) => AsonNumber::Float((i as f64) * f),
            },
            AsonNumber::Float(f) => match rhs {
                AsonNumber::Integer(i) => AsonNumber::Float(f / (i as f64)),
                AsonNumber::Float(f2) => AsonNumber::Float(f / f2),
            },
        }
    }
}

impl ops::Div for AsonNumber {
    type Output = AsonNumber;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            AsonNumber::Integer(i) => match rhs {
                AsonNumber::Integer(i2) => AsonNumber::Float((i as f64) / (i2 as f64)),
                AsonNumber::Float(f) => AsonNumber::Float((i as f64) / f),
            },
            AsonNumber::Float(f) => match rhs {
                AsonNumber::Integer(i) => AsonNumber::Float(f / (i as f64)),
                AsonNumber::Float(f2) => AsonNumber::Float(f / f2),
            },
        }
    }
}

impl AsonNumber {
    pub fn to_string(&self) -> String {
        match self {
            AsonNumber::Integer(i) => i.to_string(),
            AsonNumber::Float(f) => f.to_string(),
        }
    }
}

pub type AsonFunction = fn(Vec<AsonValue>) -> AsonValue;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum AsonValue {
    Function(AsonFunction),
    Object(HashMap<String, AsonValue>),
    Array(Vec<AsonValue>),
    String(String),
    Number(AsonNumber),
    Boolean(bool),
    Null,
}

impl AsonValue {
    pub fn is_object(&self) -> bool {
        matches!(*self, AsonValue::Object(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(*self, AsonValue::Array(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(*self, AsonValue::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(*self, AsonValue::Number(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(*self, AsonValue::Number(AsonNumber::Integer(_)))
    }

    pub fn is_float(&self) -> bool {
        matches!(*self, AsonValue::Number(AsonNumber::Float(_)))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(*self, AsonValue::Boolean(_))
    }

    pub fn is_null(&self) -> bool {
        *self == AsonValue::Null
    }

    pub fn as_object(&self) -> Option<&HashMap<String, AsonValue>> {
        match self {
            AsonValue::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn from_ason_string(s: &str) -> Result<AsonValue, String> {
        let tokens: *mut TokenList = &mut TokenList::new();
        {
            let mut lex = Lexer::new(s, unsafe{&mut *tokens});
            _ = lex.scan();
            // println!("{:#?}", unsafe{&*tokens});
        }

        let mut parser = Parser::new(unsafe{&*tokens});
        parser.parse()
    }

    pub fn to_json(&self) -> String {
        match self {
            AsonValue::Function(_) => "".into(),
            AsonValue::Object(m) => {
                let mut json = "{".to_string();
                for (k, v) in m {
                    json.push_str(&format!("\"{}\":{},", k, v.to_json()));
                }
                json.pop();
                json.push('}');
                json
            },
            AsonValue::Array(a) => {
                let mut json = "[".to_string();
                for v in a {
                    json.push_str(&format!("{},", v.to_json()));
                }
                json.pop();
                json.push(']');
                json
            },
            AsonValue::String(s) => format!("\"{}\"", s),
            AsonValue::Number(n) => n.to_string(),
            AsonValue::Boolean(b) => b.to_string(),
            AsonValue::Null => "null".to_string(),
        }
    }
}

impl fmt::Display for AsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsonValue::Function(_) => write!(f, "Function"),
            AsonValue::Object(m) => {
                write!(f, "{{")?;
                for (k, v) in m {
                    write!(f, "\"{}\":{},", k, v)?;
                }
                write!(f, "}}")
            },
            AsonValue::Array(a) => {
                write!(f, "[")?;
                for v in a {
                    write!(f, "{},", v)?;
                }
                write!(f, "]")
            },
            AsonValue::String(s) => write!(f, "\"{}\"", s),
            AsonValue::Number(n) => write!(f, "{}", n.to_string()),
            AsonValue::Boolean(b) => write!(f, "{}", b),
            AsonValue::Null => write!(f, "null"),
        }
    }
}

impl ops::Index<&str> for AsonValue {
    type Output = AsonValue;

    fn index(&self, index: &str) -> &Self::Output {
        match self {
            AsonValue::Object(m) => {
                if m.contains_key(index) {
                    &m[index]
                } else {
                    panic!("member with key '{}' is not found.", index);
                }
            },
            _ => panic!("cannot index with a string on a non-object value.")
        }
    }
}

impl ops::Index<usize> for AsonValue {
    type Output = AsonValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            AsonValue::Array(a) => {
                if index < a.len() {
                    &a[index]
                } else {
                    panic!("Index out of range.")
                }
            }
            _ => panic!("cannot index with a number on a non-array value"),
        }
    }
}

impl From<i64> for AsonValue {
    fn from(value: i64) -> Self {
        AsonValue::Number(AsonNumber::Integer(value))
    }
}

impl From<f64> for AsonValue {
    fn from(value: f64) -> Self {
        AsonValue::Number(AsonNumber::Float(value))
    }
}

impl From<String> for AsonValue {
    fn from(value: String) -> Self {
        AsonValue::String(value)
    }
}

impl From<&str> for AsonValue {
    fn from(value: &str) -> Self {
        AsonValue::String(value.into())
    }
}

impl From<bool> for AsonValue {
    fn from(value: bool) -> Self {
        AsonValue::Boolean(value)
    }
}

impl<T: Into<AsonValue>> Into<AsonValue> for Vec<T> {
    fn into(self) -> AsonValue {
        AsonValue::Array(
            self
                .into_iter()
                .map(|v| v.into())
                .collect()
        )
    }
}

impl<T: Into<AsonValue>> Into<AsonValue> for HashMap<String, T> {
    fn into(self) -> AsonValue {
        AsonValue::Object(
            self
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        )
    }
}

