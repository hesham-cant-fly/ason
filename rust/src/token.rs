#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKind {
  OpenObject, // '}'
  CloseObject, // '{'
  OpenArray, // ']'
  CloseArray, // '['
  OpenExpr, // ')'
  CloseExpr, // '('
  Colon, // ':'
  Comma, // ','
  Symbol(String),
  StringLiteral(String), // regex: "[^"]*"
  IntegerLiteral(i64), // regex: [0-9]+
  FloatLiteral(f64), // regex: [0-9]+\.[0-9]+
  True, // 'true'
  False, // 'false'
  Null, // 'null'
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub lexem: &'a str,
  pub line: usize,
  pub column: usize,
  pub index: usize
}

pub type TokenList<'a> = Vec<Token<'a>>;
