pub struct ImapClientOptions<S: AsRef<str>> {
    pub domain: S,
    pub port: u16,
}
