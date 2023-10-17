pub mod error;

use crate::error::{Error, Result};
use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_ORG_NAME: &str = "maidsafe";
const GITHUB_REPO_NAME: &str = "safe_network";

#[derive(Clone, Eq, Hash, PartialEq)]
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
pub async fn get_latest_version(release_type: &ReleaseType) -> Result<String> {
    let mut page = 1;
    let per_page = 100;
    let mut latest_release: Option<(String, DateTime<Utc>)> = None;
    let target_tag_name = *RELEASE_TYPE_CRATE_NAME_MAP.get(release_type).unwrap();
    let now = Utc::now();

    loop {
        let response = get_releases_page(page, per_page).await?;
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

        if continue_search && has_next_page(&headers).await? {
            page += 1;
        } else {
            break;
        }
    }

    let tag_name = latest_release
        .ok_or_else(|| Error::LatestReleaseNotFound(release_type.to_string()))?
        .0;
    let version = get_version_from_tag_name(&tag_name)?;
    Ok(version)
}

async fn get_releases_page(page: u32, per_page: u32) -> Result<Response> {
    let client = Client::new();
    let response = client
        .get(format!(
            "{}/repos/{}/{}/releases?page={}&per_page={}",
            GITHUB_API_URL, GITHUB_ORG_NAME, GITHUB_REPO_NAME, page, per_page
        ))
        .header("User-Agent", "request")
        .send()
        .await?;
    Ok(response)
}

async fn has_next_page(headers: &HeaderMap) -> Result<bool> {
    if let Some(links) = headers.get("link") {
        let links = links.to_str().map_err(|_| Error::HeaderLinksToStrError)?;
        Ok(links.split(',').any(|link| link.contains("rel=\"next\"")))
    } else {
        Ok(false)
    }
}

fn get_version_from_tag_name(tag_name: &str) -> Result<String> {
    let mut parts = tag_name.split('-');
    parts.next();
    let version = parts
        .next()
        .ok_or_else(|| Error::TagNameVersionParsingFailed)?
        .to_string();
    Ok(version.trim_start_matches('v').to_string())
}
