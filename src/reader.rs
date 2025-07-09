// This is free and unencumbered software released into the public domain.

use super::{MaildirIterator, MaildirMessage};
use know::datatypes::EmailMessageId;
use maildir::{MailEntryError, Maildir};
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

    pub fn iter(&self) -> impl Iterator<Item = Result<MaildirMessage, MailEntryError>> {
        MaildirIterator::new(self.maildir.list_new())
            .chain(MaildirIterator::new(self.maildir.list_cur()))
    }

    pub fn fetch(&self, mid: &EmailMessageId) -> Result<Option<MaildirMessage>, MailEntryError> {
        for entry in self.iter() {
            let message = entry?;
            if let Some(message_id) = message.message.id.as_ref() {
                if message_id == mid {
                    return Ok(Some(message));
                }
            }
        }
        Ok(None)
    }
}
