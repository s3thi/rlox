use crate::error::{RLoxError, RLoxResult};
use crate::token::{Token, TokenType};

pub struct Scanner {
    src: String,
    lexeme_start: usize,
    lexeme_current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src,
            lexeme_start: 0,
            lexeme_current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.lexeme_current >= self.src.len()
    }

    fn scan_token(&mut self) -> RLoxResult<Token> {
        if self.is_at_end() {
            return Ok(Token::new(TokenType::EOF, Some(self.line)));
        }

        let next_char = self.advance();
        match next_char {
            '(' => self.make_token_result(TokenType::LeftParen),
            ')' => self.make_token_result(TokenType::RightParen),
            '{' => self.make_token_result(TokenType::LeftBrace),
            '}' => self.make_token_result(TokenType::RightBrace),
            ',' => self.make_token_result(TokenType::Comma),
            '.' => self.make_token_result(TokenType::Dot),
            '-' => self.make_token_result(TokenType::Minus),
            '+' => self.make_token_result(TokenType::Plus),
            ';' => self.make_token_result(TokenType::Semicolon),
            '*' => self.make_token_result(TokenType::Star),
            '!' => {
                if self.advance_if_match('=') {
                    self.make_token_result(TokenType::BangEqual)
                } else {
                    self.make_token_result(TokenType::Bang)
                }
            }
            '=' => {
                if self.advance_if_match('=') {
                    self.make_token_result(TokenType::EqualEqual)
                } else {
                    self.make_token_result(TokenType::Equal)
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    self.make_token_result(TokenType::LessEqual)
                } else {
                    self.make_token_result(TokenType::Less)
                }
            }
            '>' => {
                if self.advance_if_match('=') {
                    self.make_token_result(TokenType::GreaterEqual)
                } else {
                    self.make_token_result(TokenType::Greater)
                }
            }
            '/' => {
                if self.advance_if_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    self.scan_token()
                } else {
                    self.make_token_result(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {
                self.collapse_scan();
                self.scan_token()
            }
            '\n' => {
                self.line = self.line + 1;
                self.collapse_scan();
                self.scan_token()
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

    fn parse_identifier(&mut self) -> RLoxResult<Token> {
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

        self.make_token_result(token)
    }

    fn parse_string(&mut self) -> RLoxResult<Token> {
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
        self.make_token_result(TokenType::String(collected_string))
    }

    fn parse_number(&mut self) -> RLoxResult<Token> {
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
        self.make_token_result(TokenType::Number(parsed_number))
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

    fn make_token_result(&mut self, token_type: TokenType) -> RLoxResult<Token> {
        Ok(Token::new(token_type, Some(self.line)))
    }

    fn collapse_scan(&mut self) {
        self.lexeme_start = self.lexeme_current;
    }
}

impl Iterator for Scanner {
    type Item = RLoxResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_at_end() {
            self.collapse_scan();
            Some(self.scan_token())
        } else {
            None
        }
    }
}
