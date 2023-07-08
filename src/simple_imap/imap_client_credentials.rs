use secrecy::{Secret, Zeroize};

pub struct ImapClientCredentials<S: Zeroize + AsRef<str>> {
    pub username: Secret<S>,
    pub password: Secret<S>,
}
