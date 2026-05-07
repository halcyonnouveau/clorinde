use std::sync::OnceLock;
use tokio::runtime::Runtime;
use tokio_postgres::config::SslMode;
use tokio_postgres::{Client, Config, NoTls};

use self::error::Error;

/// Creates a connection from a URL. Honours the `sslmode` query parameter:
///
/// * `disable` / `prefer` → plaintext via `NoTls` (current default behaviour).
/// * `require` / `verify-ca` / `verify-full` → rustls + the OS native trust
///   store. Requires the `tls` feature (default-on).
///
/// When the `tls` feature is disabled, a TLS-requiring `sslmode` returns
/// an [`error::Error::Tls`].
pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
    connect(url.parse()?)
}

/// Create a non-TLS connection to the container managed by Clorinde.
pub fn clorinde_conn() -> Result<Client, Error> {
    connect(
        Config::new()
            .user("postgres")
            .password("postgres")
            .host("127.0.0.1")
            .port(5435)
            .dbname("postgres")
            .clone(),
    )
}

// Global runtime for connection handling
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"))
}

/// Build a rustls `ClientConfig` whose trust store is the union of:
///
/// 1. **OS native store** (Keychain on macOS, ca-certificates on Linux,
///    Schannel on Windows) — picks up corporate CAs the user already trusts.
/// 2. **Mozilla bundle** (`webpki-roots`) — covers common public CAs
///    (Amazon Root CA 1, ISRG Root X1, …) some OS stores omit.
/// 3. **`PGSSLROOTCERT`** — libpq-compatible env var pointing at a PEM
///    bundle. Used for providers whose roots are private (e.g. AWS RDS
///    Aurora — point at `aws-rds-global-bundle.pem`).
///
/// Validates server certs by signature and chain; hostname verification is
/// performed by the TLS layer.
#[cfg(feature = "tls")]
fn build_rustls_config() -> Result<rustls::ClientConfig, Error> {
    let mut roots = rustls::RootCertStore::empty();

    // 1) OS native trust store.
    let native = rustls_native_certs::load_native_certs();
    for cert in native.certs {
        let _ = roots.add(cert);
    }

    // 2) Mozilla bundle (webpki-roots) — fills gaps the native store may miss.
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    // 3) Optional libpq-compatible custom CA bundle.
    if let Some(path) = std::env::var_os("PGSSLROOTCERT") {
        let pem = std::fs::read(&path).map_err(|e| {
            Error::Tls(format!(
                "failed to read PGSSLROOTCERT={}: {}",
                std::path::Path::new(&path).display(),
                e
            ))
        })?;
        let mut cursor = std::io::Cursor::new(pem);
        let mut added = 0usize;
        for cert in rustls_pemfile::certs(&mut cursor) {
            let cert = cert.map_err(|e| Error::Tls(format!("PGSSLROOTCERT parse: {e}")))?;
            if roots.add(cert).is_ok() {
                added += 1;
            }
        }
        if added == 0 {
            return Err(Error::Tls(
                "PGSSLROOTCERT bundle contained no valid certificates".to_string(),
            ));
        }
    }

    if roots.is_empty() {
        return Err(Error::Tls(
            "no trusted root certificates were loaded".to_string(),
        ));
    }

    Ok(rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth())
}

fn connect(config: Config) -> Result<Client, Error> {
    // Use futures::executor::block_on which works from any context
    let (tx, rx) = std::sync::mpsc::channel();

    get_runtime().spawn(async move {
        let result: Result<Client, Error> = async move {
            let client = match config.get_ssl_mode() {
                SslMode::Disable | SslMode::Prefer => {
                    let (c, conn) = config.connect(NoTls).await?;
                    tokio::spawn(conn);
                    c
                }
                #[cfg(feature = "tls")]
                _ => {
                    let tls_cfg = build_rustls_config()?;
                    let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_cfg);
                    let (c, conn) = config.connect(tls).await?;
                    tokio::spawn(conn);
                    c
                }
                #[cfg(not(feature = "tls"))]
                other => {
                    return Err(Error::Tls(format!(
                        "sslmode={other:?} requires building clorinde with the `tls` feature"
                    )));
                }
            };
            Ok(client)
        }
        .await;
        tx.send(result).unwrap();
    });

    rx.recv().unwrap()
}

// Sets the search path for the given client.
pub fn set_search_path(client: &Client, search_path: &str) -> Result<(), Error> {
    futures::executor::block_on(client.execute(&format!("SET search_path TO {search_path}"), &[]))
        .map_err(Error::from)?;
    Ok(())
}

pub(crate) mod error {
    use miette::Diagnostic;

    #[derive(Debug, thiserror::Error, Diagnostic)]
    pub enum Error {
        /// Connection / query failure surfaced by `tokio-postgres`.
        #[error("Couldn't establish a connection with the database.")]
        Connection(#[from] tokio_postgres::Error),
        /// The TLS layer (rustls + native certs) could not be set up, or
        /// the server requires TLS but clorinde was built without the
        /// `tls` feature.
        #[error("TLS error: {0}")]
        Tls(String),
    }
}
