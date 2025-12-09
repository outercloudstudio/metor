use std::fmt;
use std::str;

use crate::tokenizer::{self, TokenType};

pub enum Operator {
    BitwiseAnd,
    BitwiseOr,
    And,
    Or,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Access,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::BitwiseAnd => write!(f, "&"),
            Operator::BitwiseOr => write!(f, "|"),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanOrEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanOrEqual => write!(f, ">="),
            Operator::Assign => write!(f, "="),
            Operator::Equal => write!(f, "=="),
            Operator::Add => write!(f, "+"),
            Operator::Subtract => write!(f, "-"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Modulo => write!(f, "%"),
            Operator::Access => write!(f, "."),
        }
    }
}

pub struct OperatorNode {
    pub operator: Operator,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl OperatorNode {
    pub fn from_token(token: &tokenizer::Token) -> OperatorNode {
        return OperatorNode {
            operator: if token.content == "&" {
                Operator::BitwiseAnd
            } else if token.content == "|" {
                Operator::BitwiseOr
            } else if token.content == "<" {
                Operator::LessThan
            } else if token.content == "=" {
                Operator::Assign
            } else if token.content == ">" {
                Operator::GreaterThan
            } else if token.content == "+" {
                Operator::Add
            } else if token.content == "-" {
                Operator::Subtract
            } else if token.content == "*" {
                Operator::Multiply
            } else if token.content == "/" {
                Operator::Divide
            } else if token.content == "%" {
                Operator::Modulo
            } else if token.content == "." {
                Operator::Access
            } else {
                panic!("Encountered unknown operator {}", token.content)
            },
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Operator {}", " | ".repeat(depth), self.operator);
    }
}

impl fmt::Display for OperatorNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Operator {} {}, {} -> {}, {}",
            self.operator, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub enum Symbol {
    OpenParen,
    ClosedParen,
    Comma,
    Quote,
    SingleQuote,
    OpenCurlyBracket,
    ClosedCurlyBracket,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::OpenParen => write!(f, "("),
            Symbol::ClosedParen => write!(f, ")"),
            Symbol::Comma => write!(f, ","),
            Symbol::Quote => write!(f, "\""),
            Symbol::SingleQuote => write!(f, "'"),
            Symbol::OpenCurlyBracket => write!(f, "{{"),
            Symbol::ClosedCurlyBracket => write!(f, "}}"),
        }
    }
}

pub struct SymbolNode {
    pub symbol: Symbol,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl SymbolNode {
    pub fn from_token(token: &tokenizer::Token) -> SymbolNode {
        return SymbolNode {
            symbol: if token.content == "(" {
                Symbol::OpenParen
            } else if token.content == ")" {
                Symbol::ClosedParen
            } else if token.content == "," {
                Symbol::Comma
            } else if token.content == "\"" {
                Symbol::Quote
            } else if token.content == "'" {
                Symbol::SingleQuote
            } else if token.content == "{" {
                Symbol::OpenCurlyBracket
            } else if token.content == "}" {
                Symbol::ClosedCurlyBracket
            } else {
                panic!("Encountered unknown symbol {}", token.content)
            },
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Symbol {}", " | ".repeat(depth), self.symbol);
    }
}

impl fmt::Display for SymbolNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Symbol {} {}, {} -> {}, {}",
            self.symbol, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub enum Type {
    I32,
    U32,
    F32,
    String,
    Boolean,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "I32"),
            Type::U32 => write!(f, "U32"),
            Type::F32 => write!(f, "F32"),
            Type::String => write!(f, "String"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Void => write!(f, "Void"),
        }
    }
}

pub struct TypeNode {
    pub node_type: Type,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl TypeNode {
    pub fn from_token(token: &tokenizer::Token) -> TypeNode {
        return TypeNode {
            node_type: if token.content == "i32" {
                Type::I32
            } else if token.content == "u32" {
                Type::U32
            } else if token.content == "f32" {
                Type::F32
            } else if token.content == "string" {
                Type::String
            } else if token.content == "void" {
                Type::Void
            } else if token.content == "bool" {
                Type::Boolean
            } else {
                panic!("Encountered unknown type {}", token.content)
            },
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Type {}", " | ".repeat(depth), self.node_type);
    }
}

impl fmt::Display for TypeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type {} {}, {} -> {}, {}",
            self.node_type, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub struct BooleanNode {
    pub value: bool,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl BooleanNode {
    pub fn from_token(token: &tokenizer::Token) -> BooleanNode {
        return BooleanNode {
            value: token.content == "true",
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Boolean {}", " | ".repeat(depth), self.value);
    }
}

impl fmt::Display for BooleanNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Boolean {} {}, {} -> {}, {}",
            self.value, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub struct NumberNode {
    pub value: i64,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

const NUMBER_CHARS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl NumberNode {
    pub fn is_number(string: &String) -> bool {
        return !string.chars().any(|char| !NUMBER_CHARS.contains(&char));
    }

    pub fn from_token(token: &tokenizer::Token) -> NumberNode {
        return NumberNode {
            value: token
                .content
                .parse()
                .expect(&format!("Unknown number encountered {}", token.content)),
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Number {}", " | ".repeat(depth), self.value);
    }
}

impl fmt::Display for NumberNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Number {} {}, {} -> {}, {}",
            self.value, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub struct NameNode {
    pub value: String,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl NameNode {
    pub fn from_token(token: &tokenizer::Token) -> NameNode {
        return NameNode {
            value: token.content.clone(),
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Name {}", " | ".repeat(depth), self.value);
    }
}

impl fmt::Display for NameNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name {} {}, {} -> {}, {}",
            self.value, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub enum Keyword {
    If,
    Forever,
    Return,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::If => write!(f, "If"),
            Keyword::Forever => write!(f, "Forever"),
            Keyword::Return => write!(f, "Return"),
        }
    }
}

pub struct KeywordNode {
    pub keyword: Keyword,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl KeywordNode {
    pub fn from_token(token: &tokenizer::Token) -> KeywordNode {
        return KeywordNode {
            keyword: if token.content == "if" {
                Keyword::If
            } else if token.content == "forever" {
                Keyword::Forever
            } else if token.content == "return" {
                Keyword::Return
            } else {
                panic!("Encountered unknown keyword {}", token.content)
            },
            lines: token.lines,
            characters: token.characters,
        };
    }

    pub fn display(&self, depth: usize) -> String {
        return format!("{}Keyword {}", " | ".repeat(depth), self.keyword);
    }
}

impl fmt::Display for KeywordNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Keyword {} {}, {} -> {}, {}",
            self.keyword, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub struct BlockNode {
    pub content: Vec<Node>,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl BlockNode {
    pub fn display(&self, depth: usize) -> String {
        let mut sub_display = String::from("");

        for node in &self.content {
            sub_display += &format!("{}\n", node.display(depth + 1));
        }

        sub_display.pop().unwrap();

        return format!("{}Block\n{}", " | ".repeat(depth), sub_display);
    }
}

impl fmt::Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sub_display = String::new();

        for node in &self.content {
            sub_display += &format!("{}\n", node);
        }

        write!(
            f,
            "Block {}, {} -> {}, {}\n----\n{}----",
            self.lines.0, self.characters.0, self.lines.1, self.characters.1, sub_display
        )
    }
}

pub struct AssignmentNode {
    pub name: NameNode,
    pub value: Box<Node>,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl AssignmentNode {
    pub fn display(&self, depth: usize) -> String {
        return format!(
            "{}Assignment\n{}\n{}",
            " | ".repeat(depth),
            self.name.display(depth + 1),
            self.value.display(depth + 1)
        );
    }
}

impl fmt::Display for AssignmentNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Assignment {} = {}  {}, {} -> {}, {}",
            self.name, self.value, self.lines.0, self.characters.0, self.lines.1, self.characters.1
        )
    }
}

pub struct VariableDefinitionNode {
    pub node_type: TypeNode,
    pub assignment: AssignmentNode,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl VariableDefinitionNode {
    pub fn display(&self, depth: usize) -> String {
        return format!(
            "{}Variable Definition\n{}\n{}",
            " | ".repeat(depth),
            self.node_type.display(depth + 1),
            self.assignment.display(depth + 1)
        );
    }
}

impl fmt::Display for VariableDefinitionNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Variable Definition {} {}  {}, {} -> {}, {}",
            self.node_type,
            self.assignment,
            self.lines.0,
            self.characters.0,
            self.lines.1,
            self.characters.1
        )
    }
}

pub enum Node {
    String,
    Keyword(KeywordNode),
    Type(TypeNode),
    Operator(OperatorNode),
    Symbol(SymbolNode),
    Number(NumberNode),
    Boolean(BooleanNode),
    Name(NameNode),
    Block(BlockNode),
    Assignment(AssignmentNode),
    VariableDefinition(VariableDefinitionNode),
}

impl Node {
    pub fn get_characters(&self) -> (usize, usize) {
        match self {
            Node::String => (0, 0),
            Node::Keyword(node) => node.characters,
            Node::Type(node) => node.characters,
            Node::Operator(node) => node.characters,
            Node::Symbol(node) => node.characters,
            Node::Number(node) => node.characters,
            Node::Boolean(node) => node.characters,
            Node::Name(node) => node.characters,
            Node::Block(node) => node.characters,
            Node::Assignment(node) => node.characters,
            Node::VariableDefinition(node) => node.characters,
        }
    }

    pub fn get_lines(&self) -> (usize, usize) {
        match self {
            Node::String => (0, 0),
            Node::Keyword(node) => node.lines,
            Node::Type(node) => node.lines,
            Node::Operator(node) => node.lines,
            Node::Symbol(node) => node.lines,
            Node::Number(node) => node.lines,
            Node::Boolean(node) => node.lines,
            Node::Name(node) => node.lines,
            Node::Block(node) => node.lines,
            Node::Assignment(node) => node.lines,
            Node::VariableDefinition(node) => node.lines,
        }
    }

    pub fn display(&self, depth: usize) -> String {
        match self {
            Node::String => String::from("String"),
            Node::Keyword(node) => node.display(depth),
            Node::Type(node) => node.display(depth),
            Node::Operator(node) => node.display(depth),
            Node::Symbol(node) => node.display(depth),
            Node::Number(node) => node.display(depth),
            Node::Boolean(node) => node.display(depth),
            Node::Name(node) => node.display(depth),
            Node::Block(node) => node.display(depth),
            Node::Assignment(node) => node.display(depth),
            Node::VariableDefinition(node) => node.display(depth),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Keyword(keyword_node) => write!(f, "{}", keyword_node),
            Node::Type(type_node) => write!(f, "{}", type_node),
            Node::Name(name_node) => write!(f, "{}", name_node),
            Node::Operator(operator_node) => write!(f, "{}", operator_node),
            Node::Number(number_node) => write!(f, "{}", number_node),
            Node::Boolean(boolean_node) => write!(f, "{}", boolean_node),
            Node::Symbol(symbol_node) => write!(f, "{}", symbol_node),
            Node::Block(block_node) => write!(f, "{}", block_node),
            Node::Assignment(assignment_node) => write!(f, "{}", assignment_node),
            Node::VariableDefinition(node) => write!(f, "{}", node),
            _ => write!(f, "Unknown"),
        }
    }
}

const BOOLEAN_STRINGS: &[&str] = &["true", "false"];
const TYPE_STRINGS: &[&str] = &["i32", "u32", "f32", "string", "void", "bool"];
const OPERATOR_STRINGS: &[&str] = &["&", "|", "<", "=", ">", "+", "-", "/", "*", "%", "."];
const KEYWORD_STRINGS: &[&str] = &["if", "forever", "return"];

pub fn build_blocks(nodes: &mut Vec<Node>) {
    let mut index_stack: Vec<usize> = Vec::new();

    for index in 0..nodes.len() {
        if index >= nodes.len() {
            break;
        }

        let token = &nodes[index];

        if let Node::Symbol(symbol_node) = token
            && let Symbol::OpenCurlyBracket = symbol_node.symbol
        {
            index_stack.push(index);
        }

        if let Node::Symbol(symbol_node) = token
            && let Symbol::ClosedCurlyBracket = symbol_node.symbol
        {
            if index_stack.len() == 0 {
                continue;
            }

            let start_index = index_stack.pop().unwrap();

            let start_character = nodes[start_index].get_characters().0;
            let end_character = nodes[index].get_characters().1;

            let start_line = nodes[start_index].get_lines().0;
            let end_line = nodes[index].get_lines().1;

            let content: Vec<Node> = nodes.drain(start_index + 1..index).collect();

            nodes.splice(
                start_index..start_index + 2,
                std::iter::once(Node::Block(BlockNode {
                    content: content,
                    characters: (start_character, end_character),
                    lines: (start_line, end_line),
                })),
            );
        }
    }
}

pub fn build_assignements(nodes: &mut Vec<Node>) {
    for index in 0..nodes.len() {
        if index >= nodes.len() {
            break;
        }

        if let Node::Block(block_node) = &mut nodes[index] {
            build_assignements(&mut block_node.content);
        } else if nodes.len() >= 3 && index <= nodes.len() - 3 {
            let name_node = &nodes[index];
            let operator_node = &nodes[index + 1];

            if let Node::Name(_) = name_node
                && let Node::Operator(operator_node) = operator_node
                && let Operator::Assign = operator_node.operator
            {
                let name_node = nodes.remove(index);
                let _operator_node = nodes.remove(index);
                let value_node = nodes.remove(index);

                let lines = (name_node.get_lines().0, value_node.get_lines().1);
                let characters = (name_node.get_characters().0, value_node.get_characters().1);

                let inner_name = if let Node::Name(node) = name_node {
                    node
                } else {
                    unreachable!()
                };

                nodes.insert(
                    index,
                    Node::Assignment(AssignmentNode {
                        name: inner_name,
                        value: Box::new(value_node),
                        lines,
                        characters,
                    }),
                );
            }
        }
    }
}

pub fn build_variable_definitions(nodes: &mut Vec<Node>) {
    for index in 0..nodes.len() {
        if index >= nodes.len() {
            break;
        }

        if let Node::Block(block_node) = &mut nodes[index] {
            build_variable_definitions(&mut block_node.content);
        } else if nodes.len() >= 2 && index <= nodes.len() - 2 {
            let type_node = &nodes[index];
            let assignment_node = &nodes[index + 1];

            if let Node::Type(_) = type_node
                && let Node::Assignment(_) = assignment_node
            {
                let type_node = nodes.remove(index);
                let assignment_node = nodes.remove(index);

                let lines = (type_node.get_lines().0, assignment_node.get_lines().1);
                let characters = (
                    type_node.get_characters().0,
                    assignment_node.get_characters().1,
                );

                let inner_type = if let Node::Type(node) = type_node {
                    node
                } else {
                    unreachable!()
                };
                let inner_assignment = if let Node::Assignment(node) = assignment_node {
                    node
                } else {
                    unreachable!()
                };

                nodes.insert(
                    index,
                    Node::VariableDefinition(VariableDefinitionNode {
                        node_type: inner_type,
                        assignment: inner_assignment,
                        lines,
                        characters,
                    }),
                );
            }
        }
    }
}

pub fn build_syntax_tree(tokens: &Vec<tokenizer::Token>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    for token in tokens {
        match token.token_type {
            TokenType::Symbol => {
                if OPERATOR_STRINGS.contains(&token.content.as_str()) {
                    nodes.push(Node::Operator(OperatorNode::from_token(token)));
                } else {
                    nodes.push(Node::Symbol(SymbolNode::from_token(token)));
                }
            }
            TokenType::Unkown => {
                if TYPE_STRINGS.contains(&token.content.as_str()) {
                    nodes.push(Node::Type(TypeNode::from_token(token)));
                } else if BOOLEAN_STRINGS.contains(&token.content.as_str()) {
                    nodes.push(Node::Boolean(BooleanNode::from_token(token)));
                } else if NumberNode::is_number(&token.content) {
                    nodes.push(Node::Number(NumberNode::from_token(token)));
                } else if KEYWORD_STRINGS.contains(&token.content.as_str()) {
                    nodes.push(Node::Keyword(KeywordNode::from_token(token)));
                } else {
                    nodes.push(Node::Name(NameNode::from_token(token)));
                }
            }
            _ => {}
        }
    }

    build_blocks(&mut nodes);
    build_assignements(&mut nodes);
    build_variable_definitions(&mut nodes);

    return nodes;
}
