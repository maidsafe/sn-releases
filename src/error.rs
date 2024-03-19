// Copyright (C) 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Cannot parse file name from the URL")]
    CannotParseFilenameFromUrl,
    #[error("Unexpected response from crates.io: {0}")]
    CratesIoResponseError(u16),
    #[error(transparent)]
    DateTimeParseError(#[from] chrono::ParseError),
    #[error("Could not convert API response header links to string")]
    HeaderLinksToStrError,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Latest release not found for {0}")]
    LatestReleaseNotFound(String),
    #[error("{0}")]
    PlatformNotSupported(String),
    #[error("Could not compile the regex statement")]
    RegexError,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Release binary {0} was not found")]
    ReleaseBinaryNotFound(String),
    #[error(transparent)]
    SemVerError(#[from] semver::Error),
    #[error("Could not parse version from tag name")]
    TagNameVersionParsingFailed,
    #[error("The URL must point to a zip or gzipped tar archive")]
    UrlIsNotArchive,
    #[error(transparent)]
    ZipError(#[from] zip::result::ZipError),
}
