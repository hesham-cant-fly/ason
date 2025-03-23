use std::collections::HashMap;

use crate::ast::{AsonFunction, AsonNumber, AsonValue};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Environment {
    pub symbols: HashMap<String, AsonValue>,
}

impl Environment {
    pub fn new() -> Self {
        let mut result = Self {
            symbols: HashMap::new(),
        };
        result.define_function("+".into(), _add);
        result.define_function("-".into(), _sub);
        result.define_function("*".into(), _mul);
        result.define_function("/".into(), _div);

        result.define_function("write-line".into(), _write_line);
        result
    }

    #[allow(dead_code)]
    pub fn add_constant(&mut self, name: String, value: AsonValue) {
        self.symbols.insert(name, value);
    }

    pub fn define_function(&mut self, name: String, body: AsonFunction) {
        self.symbols.insert(name, AsonValue::Function(body));
    }

    pub fn call_fn(&self, name: &str, args: Vec<AsonValue>) -> AsonValue {
        let symbol = self.symbols.get(name).unwrap();
        match symbol {
            AsonValue::Function(f) => f(args),
            _ => panic!("Symbol is not a function"),
        }
    }
}

fn _add(value: Vec<AsonValue>) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in value {
        if let AsonValue::Number(v) = v {
            result = result + v;
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _sub(value: Vec<AsonValue>) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in value {
        if let AsonValue::Number(v) = v {
            result = result - v;
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _mul(value: Vec<AsonValue>) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in value {
        if let AsonValue::Number(v) = v {
            result = result * v;
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _div(value: Vec<AsonValue>) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in value {
        if let AsonValue::Number(v) = v {
            result = result / v;
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _write_line(value: Vec<AsonValue>) -> AsonValue {
    if value.len() < 1 {
        panic!("Expected ore or more params");
    }
    for v in value {
        print!("{} ", v);
    }
    println!();
    AsonValue::Null
}
