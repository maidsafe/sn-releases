# sn-releases

Simple crate for downloading and unpacking binaries released from the [safe_network](https://github.com/maidsafe/safe_network) repository.

## Example Usage

```rust
let temp_dir = TempDir::new("safenode")?;
let pb = Arc::new(ProgressBar::new(0));
pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
    .progress_chars("#>-"));
let pb_clone = pb.clone();
let callback: Box<dyn Fn(u64, u64) + Send + Sync> = Box::new(move |downloaded, total| {
    pb_clone.set_length(total);
    pb_clone.set_position(downloaded);
});

let release_repo = <dyn SafeReleaseRepositoryInterface>::default_config();
let archive_path = release_repo
    .download_release_from_s3(
        &ReleaseType::Safenode,
        "0.94.0",
        &Platform::LinuxMusl,
        &ArchiveType::TarGz,
        temp_dir.path(),
        &callback,
    )
    .await?;

pb.finish_with_message("Download complete");

release_repo.extract_release_archive(&archive_path, temp_dir.path())?;
```

## Testing

It's possible for users of the crate to program against the `SafeReleaseRepositoryInterface`, which can then be mocked and used in a unit test.

```rust
pub fn function_under_test(release_repo: Box<dyn SafeReleaseRepositoryInterface>) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::function_under_test;
    use async_trait::async_trait;
    use mockall::mock;
    use mockall::predicate::*;
    use sn_releases::{
        ArchiveType, Platform, ProgressCallback, ReleaseType, Result as SnReleaseResult,
        SafeReleaseRepositoryInterface,
    };
    use std::path::{Path, PathBuf};

    mock! {
        pub SafeReleaseRepository {}
        #[async_trait]
        impl SafeReleaseRepositoryInterface for SafeReleaseRepository {
            async fn get_latest_version(&self, release_type: &ReleaseType) -> SnReleaseResult<String>;
            async fn download_release_from_s3(
                &self,
                release_type: &ReleaseType,
                version: &str,
                platform: &Platform,
                archive_type: &ArchiveType,
                dest_path: &Path,
                callback: &ProgressCallback,
            ) -> SnReleaseResult<PathBuf>;
            fn extract_release_archive(&self, archive_path: &Path, dest_dir_path: &Path)
                -> SnReleaseResult<PathBuf>;
        }
    }

    #[test]
    fn test_release_repo() {
        let release_type = ReleaseType::Safe;
        let mut mock = MockSafeReleaseRepository::new();
        mock.expect_get_latest_version()
            .withf(move |arg| *arg == release_type)
            .times(1)
            .returning(|_| Ok("0.93.12".to_string()));
        let result = function_under_test(Box::new(mock));
        assert!(result.is_ok());
    }
}
```
