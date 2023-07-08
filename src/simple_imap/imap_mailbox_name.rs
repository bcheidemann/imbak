const INBOX: &str = "INBOX";

pub enum ImapMailboxName<S: AsRef<str>> {
    Inbox,
    Other(S),
}

impl<S: AsRef<str>> AsRef<str> for ImapMailboxName<S> {
    fn as_ref(&self) -> &str {
        match self {
            ImapMailboxName::Inbox => INBOX,
            ImapMailboxName::Other(s) => s.as_ref(),
        }
    }
}
