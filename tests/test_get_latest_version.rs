// Copyright (C) 2023 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use regex::Regex;
use sn_releases::{ReleaseType, SafeReleaseRepositoryInterface};

fn valid_semver_format(version: &str) -> bool {
    let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(version)
}

#[tokio::test]
async fn should_get_latest_version_of_faucet() {
    let release_type = ReleaseType::Faucet;
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let version = release_repo
        .get_latest_version(&release_type)
        .await
        .unwrap();
    assert!(valid_semver_format(&version));
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
async fn should_get_latest_version_of_safenode_rpc_client() {
    let release_type = ReleaseType::SafenodeRpcClient;
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let version = release_repo
        .get_latest_version(&release_type)
        .await
        .unwrap();
    assert!(valid_semver_format(&version));
}

#[tokio::test]
async fn should_get_latest_version_of_safenode_manager() {
    let release_type = ReleaseType::SafenodeManager;
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
