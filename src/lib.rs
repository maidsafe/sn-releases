// Copyright (C) 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

pub use crate::error::{Error, Result};

pub mod error;

use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::Client;
use semver::Version;
use serde_json::Value;
use std::collections::HashMap;
use std::env::consts::{ARCH, OS};
use std::fmt;
use std::path::{Path, PathBuf};
use tar::Archive;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

const GITHUB_API_URL: &str = "https://api.github.com";
const FAUCET_S3_BASE_URL: &str = "https://sn-faucet.s3.eu-west-2.amazonaws.com";
const NODE_LAUNCHPAD_S3_BASE_URL: &str = "https://node-launchpad.s3.eu-west-2.amazonaws.com";
const SAFE_S3_BASE_URL: &str = "https://sn-cli.s3.eu-west-2.amazonaws.com";
const SAFENODE_S3_BASE_URL: &str = "https://sn-node.s3.eu-west-2.amazonaws.com";
const SAFENODE_MANAGER_S3_BASE_URL: &str = "https://sn-node-manager.s3.eu-west-2.amazonaws.com";
const SAFENODE_RPC_CLIENT_S3_BASE_URL: &str =
    "https://sn-node-rpc-client.s3.eu-west-2.amazonaws.com";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ReleaseType {
    Faucet,
    NodeLaunchpad,
    Safe,
    Safenode,
    SafenodeManager,
    SafenodeManagerDaemon,
    SafenodeRpcClient,
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReleaseType::Faucet => "faucet",
                ReleaseType::NodeLaunchpad => "node-launchpad",
                ReleaseType::Safe => "safe",
                ReleaseType::Safenode => "safenode",
                ReleaseType::SafenodeManager => "safenode-manager",
                ReleaseType::SafenodeManagerDaemon => "safenodemand",
                ReleaseType::SafenodeRpcClient => "safenode_rpc_client",
            }
        )
    }
}

lazy_static! {
    static ref RELEASE_TYPE_CRATE_NAME_MAP: HashMap<ReleaseType, &'static str> = {
        let mut m = HashMap::new();
        m.insert(ReleaseType::Faucet, "sn_faucet");
        m.insert(ReleaseType::NodeLaunchpad, "node-launchpad");
        m.insert(ReleaseType::Safe, "sn_cli");
        m.insert(ReleaseType::Safenode, "sn_node");
        m.insert(ReleaseType::SafenodeManager, "sn-node-manager");
        m.insert(ReleaseType::SafenodeManagerDaemon, "sn-node-manager");
        m.insert(ReleaseType::SafenodeRpcClient, "sn_node_rpc_client");
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
pub trait SafeReleaseRepoActions {
    async fn get_latest_version(&self, release_type: &ReleaseType) -> Result<Version>;
    async fn download_release_from_s3(
        &self,
        release_type: &ReleaseType,
        version: &Version,
        platform: &Platform,
        archive_type: &ArchiveType,
        dest_path: &Path,
        callback: &ProgressCallback,
    ) -> Result<PathBuf>;
    async fn download_release(
        &self,
        url: &str,
        dest_dir_path: &Path,
        callback: &ProgressCallback,
    ) -> Result<PathBuf>;
    fn extract_release_archive(&self, archive_path: &Path, dest_dir_path: &Path)
        -> Result<PathBuf>;
}

impl dyn SafeReleaseRepoActions {
    pub fn default_config() -> Box<dyn SafeReleaseRepoActions> {
        Box::new(SafeReleaseRepository {
            github_api_base_url: GITHUB_API_URL.to_string(),
            faucet_base_url: FAUCET_S3_BASE_URL.to_string(),
            node_launchpad_base_url: NODE_LAUNCHPAD_S3_BASE_URL.to_string(),
            safe_base_url: SAFE_S3_BASE_URL.to_string(),
            safenode_base_url: SAFENODE_S3_BASE_URL.to_string(),
            safenode_manager_base_url: SAFENODE_MANAGER_S3_BASE_URL.to_string(),
            safenode_rpc_client_base_url: SAFENODE_RPC_CLIENT_S3_BASE_URL.to_string(),
        })
    }
}

pub struct SafeReleaseRepository {
    pub github_api_base_url: String,
    pub faucet_base_url: String,
    pub node_launchpad_base_url: String,
    pub safe_base_url: String,
    pub safenode_base_url: String,
    pub safenode_manager_base_url: String,
    pub safenode_rpc_client_base_url: String,
}

impl SafeReleaseRepository {
    fn get_base_url(&self, release_type: &ReleaseType) -> String {
        match release_type {
            ReleaseType::Faucet => self.faucet_base_url.clone(),
            ReleaseType::NodeLaunchpad => self.node_launchpad_base_url.clone(),
            ReleaseType::Safe => self.safe_base_url.clone(),
            ReleaseType::Safenode => self.safenode_base_url.clone(),
            ReleaseType::SafenodeManager => self.safenode_manager_base_url.clone(),
            ReleaseType::SafenodeManagerDaemon => self.safenode_manager_base_url.clone(),
            ReleaseType::SafenodeRpcClient => self.safenode_rpc_client_base_url.clone(),
        }
    }

    async fn download_url(
        &self,
        url: &str,
        dest_path: &PathBuf,
        callback: &ProgressCallback,
    ) -> Result<()> {
        let client = Client::new();
        let mut response = client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(Error::ReleaseBinaryNotFound(url.to_string()));
        }

        let total_size = response
            .headers()
            .get("content-length")
            .and_then(|ct_len| ct_len.to_str().ok())
            .and_then(|ct_len| ct_len.parse::<u64>().ok())
            .unwrap_or(0);

        let mut downloaded: u64 = 0;
        let mut out_file = File::create(&dest_path).await?;

        while let Some(chunk) = response.chunk().await.unwrap() {
            downloaded += chunk.len() as u64;
            out_file.write_all(&chunk).await?;
            callback(downloaded, total_size);
        }

        Ok(())
    }
}

#[async_trait]
impl SafeReleaseRepoActions for SafeReleaseRepository {
    /// Uses the crates.io API to obtain the latest version of a crate.
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
    /// - The HTTP request to crates.io API fails
    /// - The received JSON data does not have a `crate.newest_version` value
    async fn get_latest_version(&self, release_type: &ReleaseType) -> Result<Version> {
        // For the time being, the node launchpad needs to be treated as a special case, because it
        // cannot be published.
        if matches!(release_type, ReleaseType::NodeLaunchpad) {
            return Ok(Version::parse("0.1.0")?);
        }

        let crate_name = *RELEASE_TYPE_CRATE_NAME_MAP.get(release_type).unwrap();
        let url = format!("https://crates.io/api/v1/crates/{}", crate_name);

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("User-Agent", "reqwest")
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(Error::CratesIoResponseError(response.status().as_u16()));
        }

        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if let Some(version) = json["crate"]["newest_version"].as_str() {
            return Ok(Version::parse(version)?);
        }

        Err(Error::LatestReleaseNotFound(release_type.to_string()))
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
        version: &Version,
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

        let archive_name = format!(
            "{}-{}-{}.{}",
            release_type.to_string().to_lowercase(),
            version,
            platform,
            archive_ext
        );
        let archive_path = dest_path.join(archive_name);

        self.download_url(&url, &archive_path, callback).await?;

        Ok(archive_path)
    }

    async fn download_release(
        &self,
        url: &str,
        dest_dir_path: &Path,
        callback: &ProgressCallback,
    ) -> Result<PathBuf> {
        if !url.ends_with(".tar.gz") && !url.ends_with(".zip") {
            return Err(Error::UrlIsNotArchive);
        }

        let file_name = url
            .split('/')
            .last()
            .ok_or_else(|| Error::CannotParseFilenameFromUrl)?;
        let dest_path = dest_dir_path.join(file_name);

        self.download_url(url, &dest_path, callback).await?;

        Ok(dest_path)
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

pub fn get_running_platform() -> Result<Platform> {
    match OS {
        "linux" => match ARCH {
            "x86_64" => Ok(Platform::LinuxMusl),
            "armv7" => Ok(Platform::LinuxMuslArmV7),
            "arm" => Ok(Platform::LinuxMuslArm),
            "aarch64" => Ok(Platform::LinuxMuslAarch64),
            &_ => Err(Error::PlatformNotSupported(format!(
                "We currently do not have binaries for the {OS}/{ARCH} combination"
            ))),
        },
        "windows" => {
            if ARCH != "x86_64" {
                return Err(Error::PlatformNotSupported(
                    "We currently only have x86_64 binaries available for Windows".to_string(),
                ));
            }
            Ok(Platform::Windows)
        }
        "macos" => Ok(Platform::MacOs),
        &_ => Err(Error::PlatformNotSupported(format!(
            "{OS} is not currently supported"
        ))),
    }
}
