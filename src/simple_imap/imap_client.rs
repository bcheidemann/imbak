use std::net::TcpStream;

use anyhow::Context;
use imap::Client;
use native_tls::TlsStream;
use secrecy::{ExposeSecret, Zeroize};

use super::{imap_client_credentials::ImapClientCredentials, ImapClientOptions, ImapSession};

pub struct ImapClient {
    client: Client<TlsStream<TcpStream>>,
}

impl ImapClient {
    pub fn from_options<S: AsRef<str>>(options: &ImapClientOptions<S>) -> anyhow::Result<Self> {
        let tls = native_tls::TlsConnector::builder()
            .build()
            .context("Failed to build TLS connector")?;

        let domain = options.domain.as_ref();
        let addr = (options.domain.as_ref(), options.port);
        let client = imap::connect(addr, domain, &tls).context("Failed to connect")?;

        Ok(Self { client })
    }

    pub fn login<S: Zeroize + AsRef<str>>(
        self,
        credentials: &ImapClientCredentials<S>,
    ) -> anyhow::Result<ImapSession> {
        let session = self
            .client
            .login(
                credentials.username.expose_secret(),
                credentials.password.expose_secret(),
            )
            .map_err(|e| e.0)
            .context("Failed to log in")?;

        Ok(ImapSession { session })
    }
}
