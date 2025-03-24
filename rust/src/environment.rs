use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::ast::AsonValue;
use crate::ast::AsonNumber;
use crate::runtime;
use crate::runtime::AsonExpectedArgs;
use crate::runtime::AsonFunction;

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

        // Arithmatics
        result.define_function("+".into(), _add, AsonExpectedArgs::AtLeast(2));
        result.define_function("-".into(), _sub, AsonExpectedArgs::AtLeast(2));
        result.define_function("*".into(), _mul, AsonExpectedArgs::AtLeast(2));
        result.define_function("/".into(), _div, AsonExpectedArgs::AtLeast(2));

        // IO function
        result.define_function("write-line".into(), _write_line, AsonExpectedArgs::AtLeast(1));
        result.define_function("read-file-to-string".into(), _read_file_to_string, AsonExpectedArgs::Exact(1));

        result
    }

    #[allow(dead_code)]
    pub fn add_constant(&mut self, name: String, value: AsonValue) {
        self.symbols.insert(name, value);
    }

    pub fn define_function(&mut self, name: String, callback: runtime::Callback, expected_args: AsonExpectedArgs) {
        self.symbols.insert(name, AsonValue::Function(AsonFunction::new(callback, expected_args)));
    }

    pub fn call_fn(&self, name: &str, args: Vec<AsonValue>) -> AsonValue {
        let symbol = self.symbols.get(name).unwrap();
        let x = match symbol {
            AsonValue::Function(f) => f.call(args.as_slice()),
            _ => panic!("Symbol is not a function"),
        };
        match x {
            Ok(v) => v,
            Err(v) => todo!("{:?}", v)
        }
    }
}

fn _add(args: &[AsonValue]) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in args {
        if let AsonValue::Number(v) = v {
            result = result + v.clone();
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _sub(args: &[AsonValue]) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in args {
        if let AsonValue::Number(v) = v {
            result = result - v.clone();
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _mul(args: &[AsonValue]) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in args {
        if let AsonValue::Number(v) = v {
            result = result * v.clone();
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _div(args: &[AsonValue]) -> AsonValue {
    let mut result = AsonNumber::Integer(0);
    for v in args {
        if let AsonValue::Number(v) = v {
            result = result / v.clone();
        } else {
            panic!("Cannot perform an arithmatic operation on a non number value.");
        }
    }
    AsonValue::Number(result)
}

fn _write_line(args: &[AsonValue]) -> AsonValue {
    assert!(args.len() > 0);

    for v in args {
        print!("{} ", v);
    }
    println!();
    AsonValue::Null
}

fn _read_file_to_string(args: &[AsonValue]) -> AsonValue {
    assert!(args.len() == 1);

    if let AsonValue::String(ref s) = args[0] {
        let mut result = String::new();
        let mut file = match File::open(s) {
            Ok(v) => v,
            Err(e) => todo!("{:?}", e),
        };
        if let Err(e) = file.read_to_string(&mut result) {
            todo!("{:?}", e);
        }
        AsonValue::String(result)
    } else {
        panic!("read-to-string expects a string argument.");
    }
}
