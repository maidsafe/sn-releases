use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    DateTimeParseError(#[from] chrono::ParseError),
    #[error("Could not convert API response header links to string")]
    HeaderLinksToStrError,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Latest release not found for {0}")]
    LatestReleaseNotFound(String),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Could not parse version from tag name")]
    TagNameVersionParsingFailed,
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}
