use std::fmt;

#[derive(Debug)]
pub enum LoxErrorType {
    UnexpectedCharacter(char),
    UnterminatedString,
}

impl fmt::Display for LoxErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxErrorType::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character `{}`.", c,)
            }
            LoxErrorType::UnterminatedString => {
                write!(f, "String was not terminated.")
            }
        }
    }
}

#[derive(Debug)]
pub struct LoxError {
    line: u32,
    type_: LoxErrorType,
}

impl LoxError {
    pub fn new(line: u32, type_: LoxErrorType) -> LoxError {
        LoxError { line, type_ }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.type_)
    }
}
