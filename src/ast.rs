#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

#[derive(Debug)]
pub struct BinaryNode {
    left: Box<ASTNode>,
    operator: Operator,
    right: Box<ASTNode>,
}

impl BinaryNode {
    pub fn new(left: ASTNode, operator: Operator, right: ASTNode) -> Self {
        BinaryNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
struct GroupingNode {
    child: Box<ASTNode>,
}

#[derive(Debug)]
pub struct LiteralNode {
    value: Literal,
}

impl LiteralNode {
    pub fn new(value: Literal) -> Self {
        LiteralNode { value }
    }
}

#[derive(Debug)]
struct UnaryNode {
    operator: Operator,
    right: Box<ASTNode>,
}

#[derive(Debug)]
pub enum ASTNode {
    Binary(BinaryNode),
    Grouping(GroupingNode),
    Literal(LiteralNode),
    Unary(UnaryNode),
}
