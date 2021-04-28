#![cfg(feature = "download")]

use std::path::PathBuf;

use common::*;
use rustube::VideoFetcher;

#[macro_use]
mod common;

#[tokio::test]
#[ignore]
async fn download() {
    let id = random_id(SIGNATURE_CIPHER);
    let expected_path = download_path_from_id(id.as_borrowed()).await;

    let path: PathBuf = video!(id.as_owned())
        .worst_quality()
        .unwrap()
        .download()
        .await
        .unwrap();

    correct_path!(path, expected_path);
}

#[tokio::test]
#[ignore]
async fn download_age_restricted_to_dir() {
    let id = random_id(AGE_RESTRICTED);
    let expected_path = download_path_from_id(id.as_borrowed()).await;

    let path = video!(id)
        .worst_quality()
        .unwrap()
        .download_to_dir(DOWNLOAD_DIR)
        .await
        .unwrap();

    correct_path!(path, expected_path);
}

#[test]
#[ignore]
#[cfg(feature = "blocking")]
fn blocking_download_to_dir() {
    use rustube::block;
    use rustube::blocking::Video;

    let id = random_id(AGE_RESTRICTED);
    let expected_path = block!(download_path_from_id(id.as_borrowed()));

    let path = dbg!(Video::from_id(id)
        .unwrap())
        .worst_quality()
        .unwrap()
        .blocking_download_to_dir(DOWNLOAD_DIR)
        .unwrap();

    correct_path!(path, expected_path);
}

#[tokio::test]
#[ignore]
async fn download_to_dir() {
    let id = random_id(SIGNATURE_CIPHER);
    let expected_path = download_path_from_id(id.as_borrowed()).await;

    let path: PathBuf = video!(id)
        .worst_quality()
        .unwrap()
        .download_to_dir(DOWNLOAD_DIR)
        .await
        .unwrap();

    correct_path!(path, expected_path);
}

#[tokio::test]
#[ignore]
async fn download_to() {
    let id = random_id(SIGNATURE_CIPHER);
    let path = dbg!(download_path_from_id(id.as_borrowed()).await);

    let _: () = video!(id)
        .worst_quality()
        .unwrap()
        .download_to(&path)
        .await
        .unwrap();

    correct_path!(&path, path);
}
