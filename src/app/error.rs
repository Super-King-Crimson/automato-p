use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PlainTextError(pub String);

impl Display for PlainTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for PlainTextError {}