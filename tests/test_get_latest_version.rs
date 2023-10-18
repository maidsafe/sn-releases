use regex::Regex;
use sn_releases::{ReleaseType, SafeReleaseRepositoryInterface};

fn valid_semver_format(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(version)
}

#[tokio::test]
async fn should_get_latest_version_of_safe() {
    let release_type = ReleaseType::Safe;
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let version = release_repo
        .get_latest_version(&release_type)
        .await
        .unwrap();
    assert!(valid_semver_format(&version));
}

#[tokio::test]
async fn should_get_latest_version_of_safenode() {
    let release_type = ReleaseType::Safenode;
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let version = release_repo
        .get_latest_version(&release_type)
        .await
        .unwrap();
    assert!(valid_semver_format(&version));
}

#[tokio::test]
async fn should_get_latest_version_of_testnet() {
    let release_type = ReleaseType::Testnet;
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let version = release_repo
        .get_latest_version(&release_type)
        .await
        .unwrap();
    assert!(valid_semver_format(&version));
}
