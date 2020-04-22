use crate::ast::{ASTNode, BinaryNode, GroupingNode, LiteralNode, UnaryNode};
use crate::error::{RLoxError, RLoxResult};
use crate::token::{Token, TokenType};
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ASTNode {
        self.expression()
    }

    /// Top level rule for parsing expressions.
    ///
    /// Grammar:
    ///     expression -> equality
    fn expression(&mut self) -> ASTNode {
        self.equality()
    }

    /// Matches the equality testing operation.
    ///
    /// Grammar:
    ///     equality -> comparison ( ("!=" | "==") comparison )*
    fn equality(&mut self) -> ASTNode {
        let mut expr = self.comparison();

        while self.match_any(&vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = ASTNode::Binary(BinaryNode::new(expr, operator.clone(), right));
        }

        expr
    }

    fn comparison(&mut self) -> ASTNode {
        let mut expr = self.addition();

        while self.match_any(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = ASTNode::Binary(BinaryNode::new(expr, operator.clone(), right));
        }

        expr
    }

    fn addition(&mut self) -> ASTNode {
        let mut expr = self.multiplication();

        while self.match_any(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = ASTNode::Binary(BinaryNode::new(expr, operator.clone(), right));
        }

        expr
    }

    fn multiplication(&mut self) -> ASTNode {
        let mut expr = self.unary();

        while self.match_any(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = ASTNode::Binary(BinaryNode::new(expr, operator.clone(), right));
        }

        expr
    }

    fn unary(&mut self) -> ASTNode {
        if self.match_any(&vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return ASTNode::Unary(UnaryNode::new(operator.clone(), right));
        }

        self.primary()
    }

    fn primary(&mut self) -> ASTNode {
        if self.match_any(&vec![TokenType::False]) {
            return ASTNode::Literal(LiteralNode::new(TokenType::False));
        }

        if self.match_any(&vec![TokenType::True]) {
            return ASTNode::Literal(LiteralNode::new(TokenType::True));
        }

        if self.match_any(&vec![TokenType::Nil]) {
            return ASTNode::Literal(LiteralNode::new(TokenType::Nil));
        }

        if self.is_at_end() {
            return ASTNode::Error;
        }

        let next = self.advance();
        match next.token_type {
            TokenType::False => ASTNode::Literal(LiteralNode::new(TokenType::False)),
            TokenType::True => ASTNode::Literal(LiteralNode::new(TokenType::True)),
            TokenType::Nil => ASTNode::Literal(LiteralNode::new(TokenType::Nil)),
            TokenType::Number(n) => ASTNode::Literal(LiteralNode::new(TokenType::Number(n))),
            TokenType::String(s) => ASTNode::Literal(LiteralNode::new(TokenType::String(s))),
            TokenType::LeftParen => {
                let expr = self.expression();
                let next = self.consume(&TokenType::RightParen, "expected ')' after expression");
                if next.is_err() {
                    ASTNode::Error
                } else {
                    ASTNode::Grouping(GroupingNode::new(expr))
                }
            }
            _ => ASTNode::Error,
        }
    }

    /// If the next token is any one of the tokens in `token_types`, returns true
    /// and advances to the next token in the stream.
    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    /// If the next token in the stream is the token that was passed in,
    /// return true. Return false if we're at the end of the stream or if the
    /// next token is of a different type.
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    /// If we're not at the end of the token stream, advance the stream.
    /// Return the last token we saw.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    /// Is the next token an EOF token? That would mean we've reached
    /// the end of the stream.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// Returns the next token in the stream.
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Returns the previous token in the stream.
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// If the next token is of the type we expect, advance and return the token.
    /// Otherwise, return an error indicating an unexpected token.
    fn consume(&mut self, token_type: &TokenType, message: &str) -> RLoxResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = self.peek();
        Err(RLoxError::Source {
            line: token.line,
            context: token.lexeme,
            message: message.to_string(),
        })
    }
}
