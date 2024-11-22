use phf::phf_map;
use std::fmt;

use super::error::{LoxError, LoxErrorType};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map!(
    "and" => TokenKind::And,
    "class" => TokenKind::Class,
    "else" => TokenKind::Else,
    "func" => TokenKind::Func,
    "for" => TokenKind::For,
    "if" => TokenKind::If,
    "nil" => TokenKind::Nil,
    "or" => TokenKind::Or,
    "print" => TokenKind::Print,
    "return" => TokenKind::Return,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "true" => TokenKind::True,
    "var" => TokenKind::Var,
    "while" => TokenKind::While
);

#[derive(Clone)]
pub enum TokenKind {
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

impl TokenKind {
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
                Ok((TokenKind::WhiteSpace, size))
            }
            '\n' => Ok((TokenKind::NewLine, 1)),
            '(' => Ok((TokenKind::LeftParen, 1)),
            ')' => Ok((TokenKind::RightParen, 1)),
            '{' => Ok((TokenKind::LeftBrace, 1)),
            '}' => Ok((TokenKind::RightBrace, 1)),
            ',' => Ok((TokenKind::Comma, 1)),
            '.' => Ok((TokenKind::Dot, 1)),
            '-' => Ok((TokenKind::Minus, 1)),
            '+' => Ok((TokenKind::Plus, 1)),
            ';' => Ok((TokenKind::Semicolon, 1)),
            '*' => Ok((TokenKind::Star, 1)),
            '=' => {
                if value[1] == b'=' {
                    Ok((TokenKind::EqualEqual, 2))
                } else {
                    Ok((TokenKind::Equal, 1))
                }
            }
            '>' => {
                if value[1] == b'=' {
                    Ok((TokenKind::GreaterEqual, 2))
                } else {
                    Ok((TokenKind::Greater, 1))
                }
            }
            '<' => {
                if value[1] == b'=' {
                    Ok((TokenKind::LessEqual, 2))
                } else {
                    Ok((TokenKind::Less, 1))
                }
            }
            '!' => {
                if value[1] == b'=' {
                    Ok((TokenKind::BangEqual, 2))
                } else {
                    Ok((TokenKind::Bang, 1))
                }
            }
            '/' => {
                if value[1] == b'/' {
                    // we add 2 because we started from index 2, we know that
                    // the first 2 chars are `//`
                    let size = value[2..].iter().take_while(|c| **c != b'\n').count() + 2;
                    return Ok((TokenKind::Comment, size));
                } else {
                    Ok((TokenKind::Slash, 1))
                }
            }
            '"' => {
                for (i, byte) in value[1..].iter().enumerate() {
                    if *byte == b'"' {
                        return Ok((TokenKind::String, i + 2));
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
                Ok((TokenKind::Number, size))
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
                    None => Ok((TokenKind::Identifier, identifier.len())),
                }
            }
            c => Err(LoxErrorType::UnexpectedCharacter(c)),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::LeftBrace => write!(f, "LeftBrace"),
            TokenKind::RightBrace => write!(f, "RightBrace"),
            TokenKind::Comma => write!(f, "Comman"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Slash => write!(f, "Slash"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::BangEqual => write!(f, "BangEqual"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::EqualEqual => write!(f, "EqualEqual"),
            TokenKind::Greater => write!(f, "Greater"),
            TokenKind::GreaterEqual => write!(f, "GreaterEqual"),
            TokenKind::Less => write!(f, "Less"),
            TokenKind::LessEqual => write!(f, "LessEqual"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Class => write!(f, "Class"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::False => write!(f, "False"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::For => write!(f, "For"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Nil => write!(f, "Nil"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Print => write!(f, "Print"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Super => write!(f, "Super"),
            TokenKind::This => write!(f, "This"),
            TokenKind::True => write!(f, "True"),
            TokenKind::Var => write!(f, "Var"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::NewLine => write!(f, "NewLine"),
            TokenKind::WhiteSpace => write!(f, "WhiteSpace"),
        }
    }
}
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: String,
    line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} `{}` {}", self.kind, self.lexeme, self.literal)
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

        match TokenKind::from_utf8(content_slice) {
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
                    TokenKind::NewLine => self.line += 1,
                    TokenKind::String => {
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
