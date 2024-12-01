// Copyright (C) 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use ant_releases::{AntReleaseRepoActions, ArchiveType, Platform, ReleaseType};
use assert_fs::prelude::*;
use predicates::prelude::*;
use semver::Version;

const ANT_VERSION: &str = "0.1.6-rc.1";
const ANTCTL_VERSION: &str = "0.11.4-rc.1";
const ANTCTLD_VERSION: &str = "0.11.4-rc.1";
const ANTNODE_VERSION: &str = "0.112.7-rc.1";
const ANTNODE_RPC_CLIENT_VERSION: &str = "0.6.37-rc.1";
const NAT_DETECTION_VERSION: &str = "0.2.12-rc.1";
const NODE_LAUNCHPAD_VERSION: &str = "0.4.6-rc.1";

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

    let release_repo = <dyn AntReleaseRepoActions>::default_config();
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
        ReleaseType::Ant => "ant",
        ReleaseType::AntCtl => "antctl",
        ReleaseType::AntCtlDaemon => "antctld",
        ReleaseType::AntNode => "antnode",
        ReleaseType::AntNodeRpcClient => "antnode_rpc_client",
        ReleaseType::NatDetection => "nat-detection",
        ReleaseType::NodeLaunchpad => "node-launchpad",
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
/// Ant Tests
///
#[tokio::test]
async fn should_download_and_extract_autonomi_for_linux_musl() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_macos() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_macos_aarch64() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::MacOsAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_autonomi_for_windows() {
    download_and_extract(
        &ReleaseType::Ant,
        ANT_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Antnode Tests
///
#[tokio::test]
async fn should_download_and_extract_antnode_for_linux_musl() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_for_macos() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_for_windows() {
    download_and_extract(
        &ReleaseType::AntNode,
        ANTNODE_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Antctl Tests
///
#[tokio::test]
async fn should_download_and_extract_antctl_for_linux_musl() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctl_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctl_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctl_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctl_for_macos() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctl_for_windows() {
    download_and_extract(
        &ReleaseType::AntCtl,
        ANTCTL_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// Antctld Tests
///
#[tokio::test]
async fn should_download_and_extract_antctld_for_linux_musl() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctld_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctld_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctld_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctld_for_macos() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antctld_for_windows() {
    download_and_extract(
        &ReleaseType::AntCtlDaemon,
        ANTCTLD_VERSION,
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

///
/// NAT Detection Test
///
#[tokio::test]
async fn should_download_and_extract_nat_detection_for_linux_musl() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_nat_detection_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_nat_detection_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_nat_detection_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_nat_detection_for_macos() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_nat_detection_for_windows() {
    download_and_extract(
        &ReleaseType::NatDetection,
        NAT_DETECTION_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}

///
/// AntNode RPC Client Tests
///
#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_linux_musl() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_linux_musl_aarch64() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslAarch64,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_linux_musl_arm() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslArm,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_linux_musl_arm_v7() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::LinuxMuslArmV7,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_macos() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::MacOs,
        &ArchiveType::TarGz,
    )
    .await;
}

#[tokio::test]
async fn should_download_and_extract_antnode_rpc_client_for_windows() {
    download_and_extract(
        &ReleaseType::AntNodeRpcClient,
        ANTNODE_RPC_CLIENT_VERSION,
        &Platform::Windows,
        &ArchiveType::Zip,
    )
    .await;
}
