// This is free and unencumbered software released into the public domain.

use maildir::MailEntry;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaildirMessage {
    pub id: String,
}

impl From<&MailEntry> for MaildirMessage {
    fn from(entry: &MailEntry) -> Self {
        let id = entry.id().to_string();
        Self { id }
    }
}
