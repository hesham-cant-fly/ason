use std::collections::HashMap;

use crate::environment::Environment;
use crate::runtime::RuntimeError;
use crate::token::{Token, TokenKind, TokenList};
use crate::ast::{AsonExpr, AsonNumber, AsonValue};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParserError {
    msg: String,
    file: String,
    line: usize,
    column: usize,
}

impl ParserError {
    pub fn report(&self) {
        eprintln!("Error ./{}:{}:{}: {}", self.file, self.line, self.column, self.msg);
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a TokenList<'a>,
    current: usize,
    env: Environment,
    file: String,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a TokenList<'a>, file: String) -> Self {
        Parser {
            tokens,
            file,
            current: 0,
            env: Environment::new(),
        }
    }

    pub fn parse(&mut self) -> ParserResult<AsonValue> {
        self.parse_value()
    }

    fn parse_value(&mut self) -> ParserResult<AsonValue> {
        match self.advance().kind {
            TokenKind::OpenObject => self.parse_object(),
            TokenKind::OpenArray => self.parse_array(),
            TokenKind::OpenExpr => self.parse_expr(),
            TokenKind::StringLiteral(ref v) => Ok(AsonValue::String(v.clone())),
            TokenKind::IntegerLiteral(v) => Ok(AsonValue::Number(AsonNumber::Integer(v))),
            TokenKind::FloatLiteral(v) => Ok(AsonValue::Number(AsonNumber::Float(v))),
            TokenKind::True => Ok(AsonValue::Boolean(true)),
            TokenKind::False => Ok(AsonValue::Boolean(false)),
            TokenKind::Null => Ok(AsonValue::Null),
            TokenKind::Symbol(ref id) => {
                if !self.env.symbols.contains_key(id) {
                    return Err(self.report(format!("Undefined symbol: {}", id)));
                }
                let value = self.env.symbols.get(id).unwrap();
                Ok(value.clone())
            },

            _ => Err(self.report(format!("Unexpected token: {}", self.peek().lexem))),
        }
    }

    fn parse_object(&mut self) -> ParserResult<AsonValue> {
        let mut members = HashMap::<String, AsonValue>::new();
        while !self.is_at_end() {
            match self.advance().kind {
                TokenKind::CloseObject => break,
                TokenKind::StringLiteral(ref v) => {
                    let key = v.clone();
                    members.insert(key, self.parse_value()?);
                },
                TokenKind::Comma => continue,
                _ => return Err(self.report(format!("Unexpected token: {}", self.peek().lexem)))
            }
        }

        Ok(AsonValue::Object(members))
    }

    fn parse_array(&mut self) -> ParserResult<AsonValue> {
        let mut elements = Vec::<AsonValue>::new();
        while !self.is_at_end() {
            match self.peek().kind {
                TokenKind::CloseArray => {
                    _ = self.advance();
                    break;
                },
                TokenKind::Comma => _ = self.advance(),
                _ => elements.push(self.parse_value()?),
            }
        }

        Ok(AsonValue::Array(elements))
    }

    fn parse_expr(&mut self) -> ParserResult<AsonValue> {
        let expr = self.parse_expr_s()?;
        match expr.eval(&mut self.env) {
            Ok(v) => Ok(v),
            Err(e) => {
                let msg = match e {
                    RuntimeError::NotEnoughArgument { given, expected } => format!("Not Enough Arguments given, got {} expected {}.", given, expected),
                    RuntimeError::TooMuchArgument { given, expected } => format!("Too Much Arguments given, got {} expected {}.", given, expected),
                    RuntimeError::UndefinedSymbol => "Undefined Symbol.".into(),
                    RuntimeError::NotAFunction => "Not a function.".into(),
                };
                Err(self.report(msg))
            }
        }
    }

    fn parse_expr_s(&mut self) -> ParserResult<AsonExpr> {
        let mut params = Vec::new();
        while !self.is_at_end() {
            if self.peek().kind == TokenKind::CloseExpr {
                break;
            }
            let tok = self.advance();
            match tok.kind {
                TokenKind::IntegerLiteral(v) => params.push(AsonExpr::Value(v.into())),
                TokenKind::FloatLiteral(v) => params.push(AsonExpr::Value(v.into())),
                TokenKind::StringLiteral(ref v) => params.push(AsonExpr::Value(v.clone().into())),
                TokenKind::True => params.push(AsonExpr::Value(true.into())),
                TokenKind::False => params.push(AsonExpr::Value(false.into())),
                TokenKind::Null => params.push(AsonExpr::Value(AsonValue::Null)),
                TokenKind::Symbol(ref v) => params.push(AsonExpr::Symbol(v.to_string())),
                TokenKind::OpenExpr => params.push(self.parse_expr_s()?),

                _ => return Err(self.report(
                    format!(
                        "Unexpected token '{}'.\n  Expected a `number`, `string`, `boolean` (true/false), `null`, `symbol`, or a closing parenthesis `)`.",
                        self.peek().lexem
                    )
                )),
            }
        }
        _ = self.consume(TokenKind::CloseExpr, "Expected closing expressios-s '('".into())?;

        if let Some(s) = params.pop() {
            match s {
                AsonExpr::Symbol(s) => Ok(AsonExpr::ExprS(params, s)),
                _ => Err(self.report("help".into()))
            }
        } else {
            Ok(AsonExpr::None)
        }
    }

    fn peek(&self) -> &'a Token<'a> {
        if self.is_at_end() {
            &self.tokens[self.current - 1]
        } else {
            &self.tokens[self.current]
        }
    }

    fn advance(&mut self) -> &'a Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParserResult<&'a Token<'a>> {
        if self.peek().kind == kind {
            return Ok(self.advance());
        }

        Err(self.report(msg))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn report(&self, msg: String) -> ParserError {
        let current = self.peek();
        ParserError {
            msg,
            file: self.file.clone(),
            line: current.line,
            column: current.column,
        }
    }
}
