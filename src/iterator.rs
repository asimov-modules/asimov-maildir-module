// This is free and unencumbered software released into the public domain.

use super::MaildirMessage;
use maildir::MailEntries;
use std::io::Result;

pub struct MaildirIterator(MailEntries);

impl MaildirIterator {
    pub fn new(inner: MailEntries) -> Self {
        Self(inner)
    }
}

impl Iterator for MaildirIterator {
    type Item = Result<MaildirMessage>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|entry| entry.map(|entry| (&entry).into()))
    }
}
