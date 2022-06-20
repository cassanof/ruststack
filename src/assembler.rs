use std::num::ParseIntError;

use crate::parser;

#[derive(Debug)]
pub enum AssemblerError {
    Parser(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for AssemblerError {
    fn from(error: std::io::Error) -> Self {
        AssemblerError::Io(error)
    }
}

impl From<pest::error::Error<parser::Rule>> for AssemblerError {
    fn from(error: pest::error::Error<parser::Rule>) -> Self {
        AssemblerError::Parser(format!("{}", error))
    }
}

impl From<ParseIntError> for AssemblerError {
    fn from(error: ParseIntError) -> Self {
        AssemblerError::Parser(format!("{}", error))
    }
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssemblerError::Parser(s) => write!(f, "Parser error: {}", s),
            AssemblerError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for AssemblerError {}
