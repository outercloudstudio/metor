use std::fmt;

pub enum TokenType {
    WhiteSpace,
    Seperator,
    Symbol,
    Unkown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::WhiteSpace => write!(f, "White Space"),
            TokenType::Seperator => write!(f, "Seperator"),
            TokenType::Symbol => write!(f, "Symbol"),
            TokenType::Unkown => write!(f, "Unknown"),
        }
    }
}

pub struct Token {
    pub content: String,
    pub token_type: TokenType,
    pub lines: (usize, usize),
    pub characters: (usize, usize),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_type {
            TokenType::WhiteSpace => write!(
                f,
                "\'{}\' - {} {}, {} -> {}, {}",
                self.content.escape_default(),
                self.token_type,
                self.lines.0,
                self.characters.0,
                self.lines.1,
                self.characters.1
            ),
            TokenType::Seperator => write!(
                f,
                "\'{}\' - {} {}, {} -> {}, {}",
                self.content.escape_default(),
                self.token_type,
                self.lines.0,
                self.characters.0,
                self.lines.1,
                self.characters.1
            ),
            _ => write!(
                f,
                "\'{}\' - {} {}, {} -> {}, {}",
                self.content,
                self.token_type,
                self.lines.0,
                self.characters.0,
                self.lines.1,
                self.characters.1
            ),
        }
    }
}

const WHITE_SPACE_CHARACTERS: &[char] = &[' ', '\t', '\r'];
const SEPERATOR_CHARACTERS: &[char] = &['\n'];
const SYMBOL_CHARACTERS: &[char] = &[
    '+', '-', '/', '*', '=', '(', ')', ',', '"', '\'', '<', '>', '{', '}', '&', '|', '.', '%',
];

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut start_line: usize = 0;
    let mut start_character: usize = 0;

    let mut current_line: usize = 0;
    let mut current_character: usize = 0;

    let mut start_character_index: usize = 0;

    for (current_character_index, character) in code.char_indices() {
        let white_space = WHITE_SPACE_CHARACTERS.contains(&character);
        let seperator = SEPERATOR_CHARACTERS.contains(&character);
        let symbol = SYMBOL_CHARACTERS.contains(&character);

        if white_space || seperator || symbol {
            if start_character_index != current_character_index {
                tokens.push(Token {
                    content: String::from(&code[start_character_index..current_character_index]),
                    lines: (start_line, current_line),
                    characters: (start_character, current_character - 1),
                    token_type: TokenType::Unkown,
                });
            }

            tokens.push(Token {
                content: String::from(&code[current_character_index..current_character_index + 1]),
                lines: (current_line, current_line),
                characters: (current_character, current_character),
                token_type: if white_space {
                    TokenType::WhiteSpace
                } else if seperator {
                    TokenType::Seperator
                } else {
                    TokenType::Symbol
                },
            });

            start_character = current_character + 1;
            start_line = current_line;

            start_character_index = current_character_index + character.len_utf8();
        }

        current_character += 1;

        if character == '\n' {
            current_line += 1;
            current_character = 0;
        }
    }

    return tokens;
}
