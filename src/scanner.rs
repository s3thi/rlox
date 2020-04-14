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
            '"' => self.parse_string(),
            _ => {
                if next_char.is_digit(10) {
                    self.parse_number()
                } else if next_char.is_alphabetic() {
                    self.parse_identifier()
                } else {
                    Err(RLoxError::Source {
                        line: self.line,
                        location: "".to_string(),
                        message: format!("unknown character: {}", next_char),
                    })
                }
            }
        }
    }

    fn parse_identifier(&mut self) -> RLoxResult<()> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme = self.src[self.lexeme_start..self.lexeme_current].to_string();

        let token = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(lexeme.clone()),
        };

        self.add_token(token, lexeme)?;

        Ok(())
    }

    fn parse_string(&mut self) -> RLoxResult<()> {
        // Keep advancing until we read a closing quote or reach the end
        // of file.
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.advance();
        }

        // If we've read everything and we're now at the end of the file,
        // there was an unterminated string somewhere.
        if self.is_at_end() {
            return Err(RLoxError::source(
                self.line,
                "".to_string(),
                "unterminated string".to_string(),
            ));
        }

        // The closing quote.
        self.advance();

        // The arithmetic here is for exclusing the starting/ending quotes.
        let collected_string = self.src[self.lexeme_start + 1..self.lexeme_current - 1].to_string();
        let lexeme = self.src[self.lexeme_start..self.lexeme_current].to_string();
        self.add_token(TokenType::String(collected_string), lexeme)?;

        Ok(())
    }

    fn parse_number(&mut self) -> RLoxResult<()> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let lexeme = self.src[self.lexeme_start..self.lexeme_current].to_string();
        let parsed_number: f64 = lexeme.parse().unwrap();
        self.add_token(TokenType::Number(parsed_number), lexeme)?;

        Ok(())
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

    fn peek_next(&self) -> char {
        if self.lexeme_current + 1 >= self.src.len() {
            '\0'
        } else {
            self.src.chars().nth(self.lexeme_current + 1).unwrap()
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
