#![allow(unused)]

use std::path::PathBuf;

use rand::Rng;

use rustube::{Id, IdBuf};

pub const SIGNATURE_CIPHER: &[&str] = &[
    "5jlI4uzZGjU"
];
pub const PRE_SIGNED: &[&str] = &[
    "2lAe1cqCOXo",
    "QRS8MkLhQmM",
    "xQDsI2ptfgg",
    "MAoOAa_izh0",

    // todo: this video is not pre_signed
    "qG7kqns7SVM",
    // todo: idk what pre_signed is
    "JgGuRKgvWQ4",

    // youtube kids
    "JsGOGPTVkKg",

    // No external
    "hK4dUSV9erk",

    // 480p60 and 360p60
    "zCKk7HiKdko",

    // 4320p
    "UN3uF3990Q0",
    
    // 4320p60
    "CbxQWAFv7sA",
];
pub const AGE_RESTRICTED: &[&str] = &[
    "VXDsM-1McE0",
    "irauhITDrsE"
];
pub const PRIVATE: &[&str] = &[
    "m8uHb5jIGN8"
];
pub const REGION_BLOCKED: &[&str] = &[];
pub const MISSING_RECORDING: &[&str] = &[
    "5YceQ8YqYMc"
];
pub const LIVE_STREAM: &[&str] = &[
    "ASGNUnPINdM",
    "FwwgBB8l2vs"
];

pub const DOWNLOAD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/videos");

macro_rules! correct_path {
    ($path:expr, $expected:expr) => {
        dbg!(&$path);
        assert_eq!(
            $path.canonicalize().unwrap(),
            $expected.canonicalize().unwrap(),
        );
    };
}

#[cfg(feature = "descramble")]
macro_rules! video {
    ($id:expr) => {
        dbg!(
            dbg!(
                VideoFetcher::from_id($id)
                    .expect("Failed to construct a VideoFetcher ")
                    .fetch()
                    .await
                    .expect("Failed to fetch the video")
            )
                    .descramble()
                    .expect("Failed to descramble the video")
        )
    };
}

pub async fn download_path_from_id(id: Id<'_>) -> PathBuf {
    tokio::fs::create_dir_all(DOWNLOAD_DIR).await.unwrap();
    std::env::set_current_dir(DOWNLOAD_DIR).unwrap();
    let path = std::path::Path::new(DOWNLOAD_DIR)
        .join(id.as_str())
        .with_extension("mp4");
    let _ = tokio::fs::remove_file(&path).await;
    assert!(!path.is_file());
    path
}

pub fn random_id(ids: &'static [&'static str]) -> IdBuf {
    let i = rand::thread_rng()
        .gen_range(0..ids.len());

    Id::from_str(ids[i])
        .unwrap()
}

pub fn random_entry<T>(vec: &Vec<T>) -> &T {
    let i = rand::thread_rng()
        .gen_range(0..vec.len());
    &vec[i]
}
