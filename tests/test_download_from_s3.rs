use assert_fs::prelude::*;
use predicates::prelude::*;
use sn_releases::{
    download_release_from_s3, extract_release_archive, ArchiveType, Platform, ReleaseType,
};

const SAFE_VERSION: &str = "0.83.51";
const SAFENODE_VERSION: &str = "0.93.7";
const TESTNET_VERSION: &str = "0.2.213";

async fn download_and_extract(
    release_type: &ReleaseType,
    version: &str,
    platform: &Platform,
    archive_type: &ArchiveType,
) {
    let dest_dir = assert_fs::TempDir::new().unwrap().into_persistent();
    let download_dir = dest_dir.child("download_to");
    download_dir.create_dir_all().unwrap();
    let extract_dir = dest_dir.child("extract_to");
    extract_dir.create_dir_all().unwrap();

    let progress_callback = |_downloaded: u64, _total: u64| {};

    let archive_path = download_release_from_s3(
        release_type,
        version,
        platform,
        archive_type,
        &download_dir.to_path_buf(),
        &progress_callback,
    )
    .await
    .unwrap();

    let extracted_path =
        extract_release_archive(&archive_path, &extract_dir.to_path_buf()).unwrap();

    let binary_name = match release_type {
        ReleaseType::Safe => "safe",
        ReleaseType::Safenode => "safenode",
        ReleaseType::Testnet => "testnet",
    };
    let expected_binary_name = if *platform == Platform::Windows {
        format!("{}.exe", binary_name)
    } else {
        binary_name.to_string()
    };

    let binary_path = extract_dir.child(expected_binary_name);
    binary_path.assert(predicate::path::is_file());
    assert_eq!(binary_path.to_path_buf(), extracted_path);
}

///
/// Safe Tests
///
#[tokio::test]
async fn should_download_and_extract_safe_for_linux_musl() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safe_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safe_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safe_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safe_for_macos() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safe_for_windows() {
    download_and_extract(
        &ReleaseType::Safe,
        SAFE_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Safenode Tests
///
#[tokio::test]
async fn should_download_and_extract_safenode_for_linux_musl() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_for_macos() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_for_windows() {
    download_and_extract(
        &ReleaseType::Safenode,
        SAFENODE_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Testnet Tests
///
#[tokio::test]
async fn should_download_and_extract_testnet_for_linux_musl() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_testnet_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_testnet_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_testnet_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_testnet_for_macos() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_testnet_for_windows() {
    download_and_extract(
        &ReleaseType::Testnet,
        TESTNET_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}
