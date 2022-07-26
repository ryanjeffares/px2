use std::fmt;

use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "dup" => TokenType::Dup,
    "drop" => TokenType::Drop,
    "false" => TokenType::False,
    "over" => TokenType::Over,
    "println" => TokenType::PrintLn,
    "rot" => TokenType::Rot,
    "swap" => TokenType::Swap,
    "true" => TokenType::True,
};

pub struct Scanner<'a> {
    code_string: &'a String,
    code_bytes: &'a [u8],
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    start: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
    pub text: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    Dup,
    Drop,
    EndOfFile,
    Error,
    False,
    Identifier,
    Int,
    Minus,
    Over,
    Plus,
    PrintLn,
    Rot,
    Slash,
    Star,
    Swap,
    True,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token [ type: {:?}, start: {}, length: {}, line: {}, column: {}, text: '{}' ]", self.token_type, self.start, self.length, self.line, self.column, self.text)
    }
}

impl<'a> Scanner<'a> {
    pub fn new(code_string: &'a String) -> Self {
        Scanner {
            code_string,
            code_bytes: code_string.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EndOfFile);
        }

        let current_char = self.advance().unwrap(); 

        if current_char.is_ascii_digit() {
            return self.make_number();
        }

        if current_char.is_alphabetic() {
            return self.make_identifier();
        }

        match current_char {
            '+' => self.make_token(TokenType::Plus),
            '-' => self.make_token(TokenType::Minus),
            '*' => self.make_token(TokenType::Star),
            '/' => self.make_token(TokenType::Slash),
            _ => self.error_token(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        self.current += 1;
        self.column += 1;
        Some(self.code_bytes[self.current - 1] as char)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code_string.len()
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            match self.code_bytes[self.current] as char {
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                },
                ' '|'\r' => {
                    self.advance();
                },
                _ => { 
                    break;
                }
            } 
        }
    }

    fn make_number(&mut self) -> Token {
        while !self.is_at_end() && self.code_bytes[self.current].is_ascii_digit() {
            self.advance();
        }

        self.make_token(TokenType::Int) 
    }

    fn make_identifier(&mut self) -> Token {
        while !self.is_at_end() && (self.code_bytes[self.current].is_ascii_alphanumeric() || self.code_bytes[self.current] as char == '_') {
            self.advance();
        }

        let text = &self.code_string.as_str()[self.start..self.current];

        let value = KEYWORDS.get(text);
        match value {
            Some(v) => self.make_token(*v),
            None => self.make_token(TokenType::Identifier),
        }        
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let length = self.current - self.start;
        Token {
            token_type,
            start: self.start,
            length,
            line: self.line,
            column: self.column - length,
            text: &self.code_string.as_str()[self.start..self.current],
        }
    }

    fn error_token(&self) -> Token {
        Token {
            token_type: TokenType::Error,
            start: self.start,
            length: 1,
            line: self.line,
            column: self.column,
            text: "Error",
        }
    }
}
