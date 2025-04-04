use crate::token::Token;
use crate::token::TokenKind;
use crate::token::TokenList;

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    tokens: &'a mut TokenList<'a>,
    current_char: Option<char>,
    line: usize,
    column: usize,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, tokens: &'a mut TokenList<'a>) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input,
            chars,
            tokens,
            current_char,
            line: 1,
            column: 1,
            index: 0,
        }
    }

    fn is_digit(ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_symbol(ch: char) -> bool {
        matches!(ch, 'A'..='Z' | 'a'..='z' | '_' | '$' | '+' | '=' | '-' | '*' | '/' | '%' | '!' | '?' | '<' | '>')
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            self.index += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn add_simple_token(&mut self, kind: TokenKind) {
        let start_index = self.index;
        self.advance();
        self.add_token(kind, start_index);
    }

    fn add_token(&mut self, kind: TokenKind, start_index: usize) {
        let lexem = &self.input[start_index..self.index];
        self.tokens.push(Token {
            kind,
            lexem,
            line: self.line,
            column: self.column - lexem.chars().count(),
            index: start_index,
        });
    }

    fn scan_string(&mut self) -> Result<(), String> {
        // Skip the opening quote
        let start_index = self.index;
        self.advance();

        // Collect the string content
        let mut content = String::new();
        loop {
            match self.current_char {
                None => return Err("Unterminated string literal".to_string()),
                Some('"') => {
                    self.advance(); // Skip the closing quote
                    break;
                },
                Some('/') => {
                    self.advance(); // Skip the slash
                    match self.current_char {
                        Some('n') => content.push('\n'),
                        Some('r') => content.push('\r'),
                        Some('t') => content.push('\t'),
                        Some('/') => content.push('/'),
                        Some('"') => content.push('"'),
                        Some(c) => content.push(c),
                        None => return Err("Escape at end of string".to_string()),
                    }
                    self.advance();
                },
                Some(c) => {
                    content.push(c);
                    self.advance();
                }
            }
        }

        self.add_token(TokenKind::StringLiteral(content), start_index);
        Ok(())
    }

    fn scan_number(&mut self) -> Result<(), String> {
        let start_index = self.index;
        let mut has_decimal = false;

        // Collect digits before decimal point
        while let Some(c) = self.current_char {
            if Self::is_digit(c) {
                self.advance();
            } else if c == '.' {
                if has_decimal {
                    return Err("Multiple decimal points in number".to_string());
                }
                has_decimal = true;
                self.advance();
            } else {
                break;
            }
        }

        let lexem = &self.input[start_index..self.index];

        // Parse as integer or float
        if has_decimal {
            match lexem.parse::<f64>() {
                Ok(value) => self.add_token(TokenKind::FloatLiteral(value), start_index),
                Err(_) => return Err(format!("Invalid float literal: {}", lexem)),
            }
        } else {
            match lexem.parse::<i64>() {
                Ok(value) => self.add_token(TokenKind::IntegerLiteral(value), start_index),
                Err(_) => return Err(format!("Invalid integer literal: {}", lexem)),
            }
        }

        Ok(())
    }

    fn scan_symbol_or_keyword(&mut self) -> Result<(), String> {
        let start_index = self.index;

        // Collect the symbol characters
        while let Some(c) = self.current_char {
            if Self::is_symbol(c) || Self::is_digit(c) {
                self.advance();
            } else {
                break;
            }
        }

        let lexem = &self.input[start_index..self.index];

        // Check for keywords
        match lexem {
            "true" => self.add_token(TokenKind::True, start_index),
            "false" => self.add_token(TokenKind::False, start_index),
            "null" => self.add_token(TokenKind::Null, start_index),
            _ => self.add_token(TokenKind::Symbol(lexem.to_string()), start_index),
        }

        Ok(())
    }

    fn comment(&mut self) -> Result<(), String> {
        _ = self.advance();
        match self.current_char {
            Some('\\') => {
                _ = self.advance();
                while let Some(c) = self.current_char {
                    if c == '\n' {
                        _ = self.advance();
                        break;
                    }
                    _ = self.advance();
                }
                Ok(())
            }
            Some(c) => Err(format!("Unexpected character: '{}'.", c).into()),
            None => Err("Unexpected enf of file.".into())
        }
    }

    pub fn scan(&mut self) -> Result<(), String> {
        while let Some(c) = self.current_char {
            match c {
                // Skip whitespace
                c if c.is_whitespace() => self.skip_whitespace(),

                // Single-character tokens
                '}' => self.add_simple_token(TokenKind::OpenObject),
                '{' => self.add_simple_token(TokenKind::CloseObject),
                ']' => self.add_simple_token(TokenKind::OpenArray),
                '[' => self.add_simple_token(TokenKind::CloseArray),
                ')' => self.add_simple_token(TokenKind::OpenExpr),
                '(' => self.add_simple_token(TokenKind::CloseExpr),
                ':' => self.add_simple_token(TokenKind::Colon),
                ',' => self.add_simple_token(TokenKind::Comma),
                '\\' => self.comment()?,

                // String literals
                '"' => self.scan_string()?,

                // Number literals
                '0'..='9' => self.scan_number()?,

                // Symbols and keywords
                c if Self::is_symbol(c) => self.scan_symbol_or_keyword()?,

                // Unknown characters
                _ => return Err(format!("Unexpected character: {}", c)),
            }
        }

        Ok(())
    }
}
