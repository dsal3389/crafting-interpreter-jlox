use std::fmt;

use crate::error::LoxErrorType;

use super::error::LoxError;

pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comman,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl TokenType {
    /// function returns the first found token from given utf8 bytes slice, if couldn't find
    /// any token, then return an error, when token is found, return the matching token type and
    /// the length of the matching token
    pub fn from_utf8(value: &[u8]) -> Result<(Self, usize), LoxErrorType> {
        match value[0].into() {
            '(' => Ok((TokenType::LeftParen, 1)),
            ')' => Ok((TokenType::RightParen, 1)),
            '{' => Ok((TokenType::LeftBrace, 1)),
            '}' => Ok((TokenType::RightBrace, 1)),
            ',' => Ok((TokenType::Comman, 1)),
            '.' => Ok((TokenType::Dot, 1)),
            '-' => Ok((TokenType::Minus, 1)),
            '+' => Ok((TokenType::Plus, 1)),
            ';' => Ok((TokenType::Semicolon, 1)),
            '*' => Ok((TokenType::Star, 1)),
            '=' => {
                if matches!(value[1], b'=') {
                    Ok((TokenType::EqualEqual, 2))
                } else {
                    Ok((TokenType::Equal, 2))
                }
            }
            '>' => {
                if matches!(value[1], b'=') {
                    Ok((TokenType::GreaterEqual, 2))
                } else {
                    Ok((TokenType::Greater, 1))
                }
            }
            '<' => {
                if matches!(value[1], b'=') {
                    Ok((TokenType::LessEqual, 2))
                } else {
                    Ok((TokenType::Less, 1))
                }
            }
            '!' => {
                if matches!(value[1], b'=') {
                    Ok((TokenType::BangEqual, 2))
                } else {
                    Ok((TokenType::Bang, 1))
                }
            }
            c => Err(LoxErrorType::UnexpectedCharacter(c)),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comman => write!(f, "Comman"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::And => write!(f, "And"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Else => write!(f, "Else"),
            TokenType::False => write!(f, "False"),
            TokenType::Func => write!(f, "Func"),
            TokenType::For => write!(f, "For"),
            TokenType::If => write!(f, "If"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            type_,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.type_, self.lexeme, self.literal)
    }
}

pub struct Scanner {
    content: Vec<u8>,
    current: usize,
    start: usize,
    line: u32,
}

impl Scanner {
    pub fn new(content: Vec<u8>) -> Scanner {
        Scanner {
            content,
            current: 0,
            start: 0,
            line: 1,
        }
    }
}

impl Iterator for Scanner {
    type Item = Result<Token, LoxError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.content.len() {
            return None;
        }

        let content_slice = &self.content[self.current..];
        let token_type = TokenType::from_utf8(content_slice);

        if let Err(et) = token_type {
            return Some(Err(LoxError::new(self.line, et)));
        }

        let (token_type, token_size) = token_type.unwrap();
        self.current += token_size;

        let token = Token::new(token_type, String::new(), String::new(), self.line);
        Some(Ok(token))
    }
}
