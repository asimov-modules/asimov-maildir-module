// This is free and unencumbered software released into the public domain.

use core::fmt;
use maildir::MailEntryError;

#[derive(Clone, Debug)]
pub enum MaildirError {
    InvalidEntry,
    InvalidMessage,
    InvalidHeaders,
}

impl core::error::Error for MaildirError {}

impl fmt::Display for MaildirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MaildirError::*;
        match self {
            InvalidEntry => write!(f, "invalid maildir entry"),
            InvalidMessage => write!(f, "invalid maildir message"),
            InvalidHeaders => write!(f, "invalid maildir headers"),
        }
    }
}

impl From<MailEntryError> for MaildirError {
    fn from(input: MailEntryError) -> Self {
        use MailEntryError::*;
        match input {
            IOError(_) => Self::InvalidEntry,      // TODO
            ParseError(_) => Self::InvalidMessage, // TODO
            DateError(_) => Self::InvalidHeaders,  // TODO
        }
    }
}
