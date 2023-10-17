use regex::Regex;
use sn_releases::{get_latest_version, ReleaseType};
use tokio;

fn valid_semver_format(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(version)
}

#[tokio::test]
async fn test_get_latest_version_safe() {
    let release_type = ReleaseType::Safe;
    let version = get_latest_version(&release_type).await.unwrap();
    assert!(valid_semver_format(&version));
}

#[tokio::test]
async fn test_get_latest_version_safenode() {
    let release_type = ReleaseType::Safenode;
    let version = get_latest_version(&release_type).await.unwrap();
    assert!(valid_semver_format(&version));
}

#[tokio::test]
async fn test_get_latest_version_testnet() {
    let release_type = ReleaseType::Testnet;
    let version = get_latest_version(&release_type).await.unwrap();
    assert!(valid_semver_format(&version));
}
