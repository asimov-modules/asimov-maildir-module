// This is free and unencumbered software released into the public domain.

use super::{MaildirIterator, MaildirMessage};
use maildir::Maildir;
use std::{io, path::Path};

pub struct MaildirReader {
    maildir: Maildir,
}

impl MaildirReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            maildir: Maildir::from(path.as_ref().to_owned()),
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = io::Result<MaildirMessage>> {
        MaildirIterator::new(self.maildir.list_new())
            .chain(MaildirIterator::new(self.maildir.list_cur()))
    }
}
