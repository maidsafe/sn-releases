// Copyright (C) 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use assert_fs::prelude::*;
use sn_releases::error::Error;
use sn_releases::SafeReleaseRepositoryInterface;

#[tokio::test]
async fn should_download_a_custom_binary() {
    let dest_dir = assert_fs::TempDir::new().unwrap();
    let download_dir = dest_dir.child("download_to");
    download_dir.create_dir_all().unwrap();
    let downloaded_archive =
        download_dir.child("safenode-charlie-x86_64-unknown-linux-musl.tar.gz");

    let url = "https://sn-node.s3.eu-west-2.amazonaws.com/jacderida/file-upload-address/safenode-charlie-x86_64-unknown-linux-musl.tar.gz";
    let progress_callback = |_downloaded: u64, _total: u64| {};
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    release_repo
        .download_release(url, &download_dir, &progress_callback)
        .await
        .unwrap();

    downloaded_archive.assert(predicates::path::is_file());
}

#[tokio::test]
async fn should_fail_to_download_non_archive() {
    let dest_dir = assert_fs::TempDir::new().unwrap();
    let download_dir = dest_dir.child("download_to");
    download_dir.create_dir_all().unwrap();

    let url = "https://sn-node.s3.eu-west-2.amazonaws.com/jacderida/file-upload-address/safenode-charlie-x86_64-unknown-linux-musl.txt";
    let progress_callback = |_downloaded: u64, _total: u64| {};
    let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
    let result = release_repo
        .download_release(url, &download_dir, &progress_callback)
        .await;

    match result {
        Ok(_) => panic!("This test should result in a failure"),
        Err(e) => match e {
            Error::UrlIsNotArchive => {
                assert_eq!(
                    e.to_string(),
                    "The URL must point to a zip or gzipped tar archive"
                );
            }
            _ => panic!("The error type should be ReleaseBinaryNotFound"),
        },
    }
}
