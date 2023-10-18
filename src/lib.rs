pub use crate::error::{Error, Result};

pub mod error;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use tar::Archive;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_ORG_NAME: &str = "maidsafe";
const GITHUB_REPO_NAME: &str = "safe_network";
const SAFE_S3_BASE_URL: &str = "https://sn-cli.s3.eu-west-2.amazonaws.com";
const SAFENODE_S3_BASE_URL: &str = "https://sn-node.s3.eu-west-2.amazonaws.com";
const TESTNET_S3_BASE_URL: &str = "https://sn-testnet.s3.eu-west-2.amazonaws.com";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ReleaseType {
    Safe,
    Safenode,
    Testnet,
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReleaseType::Safe => "safe",
                ReleaseType::Safenode => "safenode",
                ReleaseType::Testnet => "testnet",
            }
        )
    }
}

lazy_static! {
    static ref RELEASE_TYPE_CRATE_NAME_MAP: HashMap<ReleaseType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ReleaseType::Safe, "sn_cli");
        m.insert(ReleaseType::Safenode, "sn_node");
        m.insert(ReleaseType::Testnet, "sn_testnet");
        m
    };
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Platform {
    LinuxMusl,
    LinuxMuslAarch64,
    LinuxMuslArm,
    LinuxMuslArmV7,
    MacOs,
    Windows,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::LinuxMusl => write!(f, "x86_64-unknown-linux-musl"),
            Platform::LinuxMuslAarch64 => write!(f, "aarch64-unknown-linux-musl"),
            Platform::LinuxMuslArm => write!(f, "arm-unknown-linux-musleabi"),
            Platform::LinuxMuslArmV7 => write!(f, "armv7-unknown-linux-musleabihf"),
            Platform::MacOs => write!(f, "x86_64-apple-darwin"),
            Platform::Windows => write!(f, "x86_64-pc-windows-msvc"), // This appears to be the same as the above, so I'm using the same string.
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum ArchiveType {
    TarGz,
    Zip,
}

impl fmt::Display for ArchiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArchiveType::TarGz => write!(f, "tar.gz"),
            ArchiveType::Zip => write!(f, "zip"),
        }
    }
}

pub type ProgressCallback = dyn Fn(u64, u64) + Send + Sync;

#[async_trait]
pub trait SafeReleaseRepositoryInterface {
    async fn get_latest_version(&self, release_type: &ReleaseType) -> Result<String>;
    async fn download_release_from_s3(
        &self,
        release_type: &ReleaseType,
        version: &str,
        platform: &Platform,
        archive_type: &ArchiveType,
        dest_path: &Path,
        callback: &ProgressCallback,
    ) -> Result<PathBuf>;
    fn extract_release_archive(&self, archive_path: &Path, dest_dir_path: &Path)
        -> Result<PathBuf>;
}

impl dyn SafeReleaseRepositoryInterface {
    pub fn default_config() -> Box<dyn SafeReleaseRepositoryInterface> {
        Box::new(SafeReleaseRepository {
            github_api_base_url: GITHUB_API_URL.to_string(),
            safe_base_url: SAFE_S3_BASE_URL.to_string(),
            safenode_base_url: SAFENODE_S3_BASE_URL.to_string(),
            testnet_base_url: TESTNET_S3_BASE_URL.to_string(),
        })
    }
}

pub struct SafeReleaseRepository {
    pub github_api_base_url: String,
    pub safe_base_url: String,
    pub safenode_base_url: String,
    pub testnet_base_url: String,
}

impl SafeReleaseRepository {
    fn get_base_url(&self, release_type: &ReleaseType) -> String {
        match release_type {
            ReleaseType::Safe => self.safe_base_url.clone(),
            ReleaseType::Safenode => self.safenode_base_url.clone(),
            ReleaseType::Testnet => self.testnet_base_url.clone(),
        }
    }

    async fn get_releases_page(&self, page: u32, per_page: u32) -> Result<Response> {
        let client = Client::new();
        let response = client
            .get(format!(
                "{}/repos/{}/{}/releases?page={}&per_page={}",
                self.github_api_base_url, GITHUB_ORG_NAME, GITHUB_REPO_NAME, page, per_page
            ))
            .header("User-Agent", "request")
            .send()
            .await?;
        Ok(response)
    }

    async fn has_next_page(&self, headers: &HeaderMap) -> Result<bool> {
        if let Some(links) = headers.get("link") {
            let links = links.to_str().map_err(|_| Error::HeaderLinksToStrError)?;
            Ok(links.split(',').any(|link| link.contains("rel=\"next\"")))
        } else {
            Ok(false)
        }
    }

    fn get_version_from_tag_name(&self, tag_name: &str) -> Result<String> {
        let mut parts = tag_name.split('-');
        parts.next();
        let version = parts
            .next()
            .ok_or_else(|| Error::TagNameVersionParsingFailed)?
            .to_string();
        Ok(version.trim_start_matches('v').to_string())
    }
}

#[async_trait]
impl SafeReleaseRepositoryInterface for SafeReleaseRepository {
    /// Gets the latest version for a specified binary in the `safe_network` repository.
    ///
    /// Each release in the repository is checked, starting from the most recent. The `safe_network`
    /// repository is a workspace to which many binaries are released, so it's not possible to use the
    /// more straight forward Github API which simply returns the latest release, since that's going to
    /// be the version number for one of many binaries.
    ///
    /// During the search, if a release is found that was created more than 14 days ago, the function
    /// will stop searching through older releases, which will avoid fetching further pages from the
    /// Github API.
    ///
    /// # Arguments
    ///
    /// * `release_type` - A reference to a `ReleaseType` enum specifying the type of release to look for.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `String` with the latest version number in the semantic format.
    /// Otherwise, returns an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request to GitHub API fails
    /// - The received JSON data from the API is not as expected
    /// - No releases are found that match the specified `ReleaseType`
    async fn get_latest_version(&self, release_type: &ReleaseType) -> Result<String> {
        let mut page = 1;
        let per_page = 100;
        let mut latest_release: Option<(String, DateTime<Utc>)> = None;
        let target_tag_name = *RELEASE_TYPE_CRATE_NAME_MAP.get(release_type).unwrap();
        let now = Utc::now();

        loop {
            let response = self.get_releases_page(page, per_page).await?;
            let headers = response.headers().clone();
            let releases = response.json::<Value>().await?;

            let mut continue_search = true;
            if let Value::Array(releases) = releases {
                for release in releases {
                    if let Value::Object(release) = release {
                        if let (Some(Value::String(tag_name)), Some(Value::String(created_at))) =
                            (release.get("tag_name"), release.get("created_at"))
                        {
                            let created_at = created_at.parse::<DateTime<Utc>>()?;
                            if tag_name.starts_with(target_tag_name) {
                                match latest_release {
                                    Some((_, date)) if created_at > date => {
                                        latest_release = Some((tag_name.clone(), created_at));
                                    }
                                    None => {
                                        latest_release = Some((tag_name.clone(), created_at));
                                    }
                                    _ => {}
                                }
                            }

                            if now.signed_duration_since(created_at) > Duration::days(14) {
                                continue_search = false;
                                break;
                            }
                        }
                    }
                }
            }

            if continue_search && self.has_next_page(&headers).await? {
                page += 1;
            } else {
                break;
            }
        }

        let tag_name = latest_release
            .ok_or_else(|| Error::LatestReleaseNotFound(release_type.to_string()))?
            .0;
        let version = self.get_version_from_tag_name(&tag_name)?;
        Ok(version)
    }

    /// Downloads a release binary archive from S3.
    ///
    /// # Arguments
    ///
    /// - `release_type`: The type of release.
    /// - `version`: The version of the release.
    /// - `platform`: The target platform.
    /// - `archive_type`: The type of archive (e.g., tar.gz, zip).
    /// - `dest_path`: The directory where the downloaded archive will be stored.
    /// - `callback`: A callback function that can be used for download progress.
    ///
    /// # Returns
    ///
    /// A `Result` with `PathBuf` indicating the full path of the downloaded archive, or an error if
    /// the download or file write operation fails.
    async fn download_release_from_s3(
        &self,
        release_type: &ReleaseType,
        version: &str,
        platform: &Platform,
        archive_type: &ArchiveType,
        dest_path: &Path,
        callback: &ProgressCallback,
    ) -> Result<PathBuf> {
        let archive_ext = archive_type.to_string();
        let url = format!(
            "{}/{}-{}-{}.{}",
            self.get_base_url(release_type),
            release_type.to_string().to_lowercase(),
            version,
            platform,
            archive_type
        );

        let client = Client::new();
        let mut response = client.get(&url).send().await.unwrap();

        let total_size = response
            .headers()
            .get("content-length")
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);

        let mut downloaded: u64 = 0;
        let archive_name = format!(
            "{}-{}-{}.{}",
            release_type.to_string().to_lowercase(),
            version,
            platform,
            archive_ext
        );
        let archive_path = dest_path.join(archive_name);
        let mut out_file = File::create(&archive_path).await?;

        while let Some(chunk) = response.chunk().await.unwrap() {
            downloaded += chunk.len() as u64;
            out_file.write_all(&chunk).await?;
            callback(downloaded, total_size);
        }

        Ok(archive_path)
    }

    /// Extracts a release binary archive.
    ///
    /// The archive will include a single binary file.
    ///
    /// # Arguments
    ///
    /// - `archive_path`: The path of the archive file to extract.
    /// - `dest_dir`: The directory where the archive should be extracted.
    ///
    /// # Returns
    ///
    /// A `Result` with `PathBuf` indicating the full path of the extracted binary.
    fn extract_release_archive(
        &self,
        archive_path: &Path,
        dest_dir_path: &Path,
    ) -> Result<PathBuf> {
        if !archive_path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Archive not found at: {:?}", archive_path),
            )));
        }

        if archive_path.extension() == Some(std::ffi::OsStr::new("gz")) {
            let archive_file = std::fs::File::open(archive_path)?;
            let tarball = flate2::read::GzDecoder::new(archive_file);
            let mut archive = Archive::new(tarball);
            if let Some(file) = (archive.entries()?).next() {
                let mut file = file?;
                let out_path = dest_dir_path.join(file.path()?);
                file.unpack(&out_path)?;
                return Ok(out_path);
            }
        } else if archive_path.extension() == Some(std::ffi::OsStr::new("zip")) {
            let archive_file = std::fs::File::open(archive_path)?;
            let mut archive = ZipArchive::new(archive_file)?;
            if let Some(i) = (0..archive.len()).next() {
                let mut file = archive.by_index(i)?;
                let out_path = dest_dir_path.join(file.name());
                if file.name().ends_with('/') {
                    std::fs::create_dir_all(&out_path)?;
                } else {
                    let mut outfile = std::fs::File::create(&out_path)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
                return Ok(out_path);
            }
        } else {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported archive format",
            )));
        }

        Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to extract archive",
        )))
    }
}
