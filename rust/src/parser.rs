use std::collections::HashMap;

use crate::environment::Environment;
use crate::token::{Token, TokenKind, TokenList};
use crate::ast::{AsonExpr, AsonNumber, AsonValue};

type ParserResult<T> = Result<T, String>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a TokenList<'a>,
    current: usize,
    env: Environment
}

impl<'a> Parser<'a> {

    pub fn new(tokens: &'a TokenList<'a>) -> Self {
        Parser {
            tokens,
            current: 0,
            env: Environment::new()
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

            _ => Err(format!("Unexpected token: {}", self.peek().lexem)),
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
                _ => return Err(format!("Unexpected token: {}", self.peek().lexem))
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
        Ok(expr.eval(&mut self.env))
    }

    fn parse_expr_s(&mut self) -> ParserResult<AsonExpr> {
        let mut params = Vec::new();
        while self.peek().kind != TokenKind::CloseExpr {
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

                _ => return Err(format!("Expected expression."))
            }
        }
        _ = self.advance();

        if let Some(s) = params.pop() {
            match s {
                AsonExpr::Symbol(s) => Ok(AsonExpr::ExprS(params, s)),
                _ => Err(format!("help"))
            }
        } else {
            Ok(AsonExpr::None)
        }
    }

    fn peek(&self) -> &'a Token<'a> {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &'a Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    #[allow(dead_code)]
    fn consume(&mut self, _kind: TokenKind, _msg: &'static str) {
        unimplemented!();
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
