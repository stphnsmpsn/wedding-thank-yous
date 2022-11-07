use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("CSV Error: `{0}`")]
    Csv(#[from] csv::Error),
    #[error("SMTP Transport Error: `{0}`")]
    SmtpTransport(#[from] lettre::transport::smtp::Error),
    #[error("Address Error: `{0}`")]
    Address(#[from] lettre::address::AddressError),
    #[error("Lettre Error: `{0}`")]
    Lettre(#[from] lettre::error::Error),
    #[error("I/O Error: `{0}`")]
    IO(#[from] std::io::Error),
    #[error("Content Type Error: `{0}`")]
    ContentType(#[from] lettre::message::header::ContentTypeErr),
    #[error("Error opening image!")]
    Image,
}
