use phf::phf_map;
use std::fmt;

use super::error::{LoxError, LoxErrorType};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map!(
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "func" => TokenType::Func,
    "for" => TokenType::For,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While
);

#[derive(Clone)]
pub enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
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

    // other unique
    Comment,
    NewLine,
    WhiteSpace,
}

impl TokenType {
    /// function returns the first found token from given utf8 bytes slice, if couldn't find
    /// any token, then return an error, when token is found, return the matching token type and
    /// the length of the matching token
    pub fn from_utf8(value: &[u8]) -> Result<(Self, usize), LoxErrorType> {
        match value[0].into() {
            '\r' | '\t' | ' ' => {
                let size = value[1..]
                    .iter()
                    .take_while(|c| matches!(c, b'\r' | b'\t' | b' '))
                    .count()
                    + 1;
                Ok((TokenType::WhiteSpace, size))
            }
            '\n' => Ok((TokenType::NewLine, 1)),
            '(' => Ok((TokenType::LeftParen, 1)),
            ')' => Ok((TokenType::RightParen, 1)),
            '{' => Ok((TokenType::LeftBrace, 1)),
            '}' => Ok((TokenType::RightBrace, 1)),
            ',' => Ok((TokenType::Comma, 1)),
            '.' => Ok((TokenType::Dot, 1)),
            '-' => Ok((TokenType::Minus, 1)),
            '+' => Ok((TokenType::Plus, 1)),
            ';' => Ok((TokenType::Semicolon, 1)),
            '*' => Ok((TokenType::Star, 1)),
            '=' => {
                if value[1] == b'=' {
                    Ok((TokenType::EqualEqual, 2))
                } else {
                    Ok((TokenType::Equal, 1))
                }
            }
            '>' => {
                if value[1] == b'=' {
                    Ok((TokenType::GreaterEqual, 2))
                } else {
                    Ok((TokenType::Greater, 1))
                }
            }
            '<' => {
                if value[1] == b'=' {
                    Ok((TokenType::LessEqual, 2))
                } else {
                    Ok((TokenType::Less, 1))
                }
            }
            '!' => {
                if value[1] == b'=' {
                    Ok((TokenType::BangEqual, 2))
                } else {
                    Ok((TokenType::Bang, 1))
                }
            }
            '/' => {
                if value[1] == b'/' {
                    // we add 2 because we started from index 2, we know that
                    // the first 2 chars are `//`
                    let size = value[2..].iter().take_while(|c| **c != b'\n').count() + 2;
                    return Ok((TokenType::Comment, size));
                } else {
                    Ok((TokenType::Slash, 1))
                }
            }
            '"' => {
                for (i, byte) in value[1..].iter().enumerate() {
                    if *byte == b'"' {
                        return Ok((TokenType::String, i + 2));
                    }
                }
                Err(LoxErrorType::UnterminatedString)
            }
            '0'..'9' => {
                let mut post_dot = false;
                let mut size = 0usize;

                for byte in value[1..].iter() {
                    // look for numbers and dot floating points
                    if *byte == b'.' {
                        // if current char is a `.` (floating point) then we check
                        // if we are already `post_dot` because a floating point cannot
                        // have multiple dots
                        if post_dot {
                            break;
                        }
                        post_dot = true;
                    } else if !byte.is_ascii_digit() {
                        break;
                    }
                    size += 1;
                }
                Ok((TokenType::Number, size))
            }
            'a'..'z' | 'A'..'Z' | '_' => {
                let identifier = String::from_utf8(
                    value
                        .iter()
                        .take_while(|c| matches!(c, b'a'..b'z' | b'A'..b'Z' | b'_'))
                        .map(|c| *c)
                        .collect(),
                )
                .unwrap();

                match KEYWORDS.get(&identifier) {
                    Some(t) => Ok(((*t).clone(), identifier.len())),
                    None => Ok((TokenType::Identifier, identifier.len())),
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
            TokenType::Comma => write!(f, "Comman"),
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
            TokenType::Comment => write!(f, "Comment"),
            TokenType::NewLine => write!(f, "NewLine"),
            TokenType::WhiteSpace => write!(f, "WhiteSpace"),
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
        write!(f, "{} `{}` {}", self.type_, self.lexeme, self.literal)
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

    /// returns the next available token, in case of error, return
    /// a `LoxError`, the iterator will return `None` when there are no
    /// more tokens to process
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.content.len() {
            return None;
        }

        let content_slice = &self.content[self.current..];

        match TokenType::from_utf8(content_slice) {
            Ok((token_type, token_size)) => {
                // get the lexeme string based on the returned `token_size`
                let lexeme =
                    unsafe { String::from_utf8_unchecked(content_slice[..token_size].to_vec()) };

                // update the current (cursor) to point to the next char
                // based on the token size
                self.current += token_size;

                // some tokens have special meaning to the scanner, in
                // this match case we handle those special cases
                match token_type {
                    TokenType::NewLine => self.line += 1,
                    TokenType::String => {
                        // since lox supports multi line strings, we need to couldn't how many
                        // new lines there are in the `lexeme` and update the scanner `line`
                        // property
                        let new_lines = lexeme.chars().filter(|c| *c == '\n').count();
                        self.line += new_lines as u32;
                    }
                    _ => {}
                }

                let token = Token::new(token_type, lexeme, String::new(), self.line);
                Some(Ok(token))
            }
            Err(error_type) => Some(Err(LoxError::new(self.line, error_type))),
        }
    }
}
