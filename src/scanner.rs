use crate::error::{RLoxError, RLoxResult};
use crate::token::{Token, TokenType};

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    lexeme_start: usize,
    lexeme_current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src,
            tokens: vec![],
            lexeme_start: 0,
            lexeme_current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> RLoxResult<Vec<Token>> {
        while !self.is_at_end() {
            self.lexeme_start = self.lexeme_current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.lexeme_current >= self.src.len()
    }

    fn scan_token(&mut self) -> RLoxResult<()> {
        let next_char = self.advance();
        match next_char {
            '(' => self.add_token(TokenType::LeftParen, self.build_lexeme_string()),
            ')' => self.add_token(TokenType::RightParen, self.build_lexeme_string()),
            '{' => self.add_token(TokenType::LeftBrace, self.build_lexeme_string()),
            '}' => self.add_token(TokenType::RightBrace, self.build_lexeme_string()),
            ',' => self.add_token(TokenType::Comma, self.build_lexeme_string()),
            '.' => self.add_token(TokenType::Dot, self.build_lexeme_string()),
            '-' => self.add_token(TokenType::Minus, self.build_lexeme_string()),
            '+' => self.add_token(TokenType::Plus, self.build_lexeme_string()),
            ';' => self.add_token(TokenType::Semicolon, self.build_lexeme_string()),
            '*' => self.add_token(TokenType::Star, self.build_lexeme_string()),
            '!' => {
                if self.advance_if_match('=') {
                    self.add_token(TokenType::BangEqual, self.build_lexeme_string())
                } else {
                    self.add_token(TokenType::Bang, self.build_lexeme_string())
                }
            }
            '=' => {
                if self.advance_if_match('=') {
                    self.add_token(TokenType::EqualEqual, self.build_lexeme_string())
                } else {
                    self.add_token(TokenType::Equal, self.build_lexeme_string())
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    self.add_token(TokenType::LessEqual, self.build_lexeme_string())
                } else {
                    self.add_token(TokenType::Less, self.build_lexeme_string())
                }
            }
            '>' => {
                if self.advance_if_match('=') {
                    self.add_token(TokenType::GreaterEqual, self.build_lexeme_string())
                } else {
                    self.add_token(TokenType::Greater, self.build_lexeme_string())
                }
            }
            '/' => {
                if self.advance_if_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash, self.build_lexeme_string())
                }
            }
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line = self.line + 1;
                Ok(())
            }
            _ => Err(RLoxError::Source {
                line: self.line,
                location: "".to_string(),
                message: format!("unknown character: {}", next_char),
            }),
        }
    }

    fn advance(&mut self) -> char {
        self.lexeme_current = self.lexeme_current + 1;
        self.src.chars().nth(self.lexeme_current - 1).unwrap()
    }

    fn advance_if_match(&mut self, test: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.src.chars().nth(self.lexeme_current).unwrap() != test {
            return false;
        }

        self.lexeme_current = self.lexeme_current + 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.src.chars().nth(self.lexeme_current).unwrap()
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) -> RLoxResult<()> {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
        Ok(())
    }

    fn build_lexeme_string(&self) -> String {
        self.src[self.lexeme_start..self.lexeme_current].to_string()
    }
}
