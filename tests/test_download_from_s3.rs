// Copyright (C) 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use assert_fs::prelude::*;
use predicates::prelude::*;
use semver::Version;
use sn_releases::{ArchiveType, Platform, ReleaseType, SafeReleaseRepoActions};

const FAUCET_VERSION: &str = "0.1.98";
const NODE_LAUNCHPAD_VERSION: &str = "0.1.0";
const SAFE_VERSION: &str = "0.83.51";
const SAFENODE_VERSION: &str = "0.93.7";
const SAFENODE_MANAGER_VERSION: &str = "0.1.8";
const SAFENODE_MANAGERD_VERSION: &str = "0.4.1";
const SAFENODE_RPC_CLIENT_VERSION: &str = "0.1.40";

async fn download_and_extract(
    release_type: &ReleaseType,
    version: &str,
    platform: &Platform,
    archive_type: &ArchiveType,
) {
    let dest_dir = assert_fs::TempDir::new().unwrap();
    let download_dir = dest_dir.child("download_to");
    download_dir.create_dir_all().unwrap();
    let extract_dir = dest_dir.child("extract_to");
    extract_dir.create_dir_all().unwrap();

    let progress_callback = |_downloaded: u64, _total: u64| {};

    let release_repo = <dyn SafeReleaseRepoActions>::default_config();
    let archive_path = release_repo
        .download_release_from_s3(
            release_type,
            &Version::parse(version).unwrap(),
            platform,
            archive_type,
            &download_dir,
            &progress_callback,
        )
        .await
        .unwrap();

    let extracted_path = release_repo
        .extract_release_archive(&archive_path, &extract_dir)
        .unwrap();

    let binary_name = match release_type {
        ReleaseType::Faucet => "faucet",
        ReleaseType::NodeLaunchpad => "node-launchpad",
        ReleaseType::Safe => "safe",
        ReleaseType::Safenode => "safenode",
        ReleaseType::SafenodeManager => "safenode-manager",
        ReleaseType::SafenodeManagerDaemon => "safenodemand",
        ReleaseType::SafenodeRpcClient => "safenode_rpc_client",
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
/// Safenode RPC client tests
///
#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_linux_musl() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_macos() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_rpc_client_for_windows() {
    download_and_extract(
        &ReleaseType::SafenodeRpcClient,
        SAFENODE_RPC_CLIENT_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Node Manager Tests
///
#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_linux_musl() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_macos() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenode_manager_for_windows() {
    download_and_extract(
        &ReleaseType::SafenodeManager,
        SAFENODE_MANAGER_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Faucet Tests
///
#[tokio::test]
async fn should_download_and_extract_faucet_for_linux_musl() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_faucet_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_faucet_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_faucet_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_faucet_for_macos() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_faucet_for_windows() {
    download_and_extract(
        &ReleaseType::Faucet,
        FAUCET_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Node Manager Daemon Tests
///
#[tokio::test]
async fn should_download_and_extract_safenodemand_for_linux_musl() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenodemand_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenodemand_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenodemand_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenodemand_for_macos() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_safenodemand_for_windows() {
    download_and_extract(
        &ReleaseType::SafenodeManagerDaemon,
        SAFENODE_MANAGERD_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Node Launchpad Tests
///
#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_linux_musl() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_macos() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_node_launchpad_for_windows() {
    download_and_extract(
        &ReleaseType::NodeLaunchpad,
        NODE_LAUNCHPAD_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}
