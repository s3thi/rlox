use crate::token::TokenType;

#[derive(Debug)]
pub struct BinaryNode {
    left: Box<ASTNode>,
    operator: TokenType,
    right: Box<ASTNode>,
}

impl BinaryNode {
    pub fn new(left: ASTNode, operator: TokenType, right: ASTNode) -> Self {
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
    value: TokenType,
}

impl LiteralNode {
    pub fn new(value: TokenType) -> Self {
        LiteralNode { value }
    }
}

#[derive(Debug)]
struct UnaryNode {
    operator: TokenType,
    child: Box<ASTNode>,
}

#[derive(Debug)]
pub enum ASTNode {
    Binary(BinaryNode),
    Grouping(GroupingNode),
    Literal(LiteralNode),
    Unary(UnaryNode),
}

fn make_graphviz_label(node: &ASTNode, depth: u32) -> String {
    match node {
        ASTNode::Binary(node) => format!("{}_{}", node.operator, depth),
        ASTNode::Grouping(_) => format!("group_{}", depth),
        ASTNode::Literal(node) => format!("{}_{}", node.value, depth),
        ASTNode::Unary(node) => format!("{}_{}", node.operator, depth),
    }
}

fn pretty_print_recursive(node: &ASTNode, acc: &mut String, depth: u32) {
    let label = make_graphviz_label(node, depth);
    match node {
        ASTNode::Binary(bin_node) => {
            acc.push_str(&format!("\"{}\"[label=\"{}\"];\n", label, bin_node.operator));

            pretty_print_recursive(&bin_node.left, acc, depth + 1);
            pretty_print_recursive(&bin_node.right, acc, depth + 1);

            let label_left = make_graphviz_label(&bin_node.left, depth + 1);
            let label_right = make_graphviz_label(&bin_node.right, depth + 1);
            acc.push_str(&format!("\"{}\" -> \"{}\";\n", label, label_left));
            acc.push_str(&format!("\"{}\" -> \"{}\";\n", label, label_right));
        }
        ASTNode::Grouping(group_node) => {
            acc.push_str(&format!("\"{}\"[label=\"{}\"];\n", label, "()"));
            pretty_print_recursive(&group_node.child, acc, depth + 1);

            let label_child = make_graphviz_label(&group_node.child, depth + 1);
            acc.push_str(&format!("\"{}\" -> \"{}\";\n", label, label_child));
        }
        ASTNode::Literal(lit_node) => {
            acc.push_str(&format!("\"{}\"[label=\"{}\"];\n", label, lit_node.value));
        }
        ASTNode::Unary(unary_node) => {
            acc.push_str(&format!("\"{}\"[label=\"{}\"];\n", label, unary_node.operator));
            pretty_print_recursive(&unary_node.child, acc, depth + 1);

            let label_child = make_graphviz_label(&unary_node.child, depth + 1);
            acc.push_str(&format!("\"{}\" -> \"{}\";\n", label, label_child));
        }
    };
}

pub trait PrettyPrinter {
    fn pretty_print(&self) -> String;
}

impl PrettyPrinter for ASTNode {
    fn pretty_print(&self) -> String {
        let mut pretty = String::new();
        pretty.push_str("digraph G {\n");
        pretty_print_recursive(self, &mut pretty, 0);
        pretty.push_str("}");
        pretty
    }
}
