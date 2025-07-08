// This is free and unencumbered software released into the public domain.

use super::MaildirMessage;
use maildir::{MailEntries, MailEntryError};

pub struct MaildirIterator(MailEntries);

impl MaildirIterator {
    pub fn new(inner: MailEntries) -> Self {
        Self(inner)
    }
}

impl Iterator for MaildirIterator {
    type Item = Result<MaildirMessage, MailEntryError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(Err(err)) => Some(Err(MailEntryError::IOError(err))),
            Some(Ok(entry)) => Some(entry.try_into()),
        }
    }
}
