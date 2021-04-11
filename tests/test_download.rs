use std::path::PathBuf;

use rand::Rng;

use rustube::{block, Error, Id, IdBuf, Video, VideoFetcher};
use rustube::video_info::player_response::playability_status::PlayabilityStatus;

const SIGNATURE_CIPHER: &[&str] = &[
    "5jlI4uzZGjU"
];
const PRE_SIGNED: &[&str] = &[
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
];
const AGE_RESTRICTED: &[&str] = &[
    "VXDsM-1McE0",
    "irauhITDrsE"
];
const PRIVATE: &[&str] = &[
    "m8uHb5jIGN8"
];
const REGION_BLOCKED: &[&str] = &[];
const MISSING_RECORDING: &[&str] = &[
    "5YceQ8YqYMc"
];
const LIVE_STREAM: &[&str] = &[
    "ASGNUnPINdM"
];

const DOWNLOAD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/videos");

macro_rules! correct_path {
    ($path:expr, $expected:expr) => {
        dbg!(&$path);
        assert_eq!(
            $path.canonicalize().unwrap(),
            $expected.canonicalize().unwrap(),
        );
    };
}

async fn download_path_from_id(id: Id<'_>) -> PathBuf {
    std::env::set_current_dir(DOWNLOAD_DIR).unwrap();
    let path = std::path::Path::new(DOWNLOAD_DIR)
        .join(id.as_str())
        .with_extension("mp4");
    let _ = tokio::fs::remove_file(&path).await;
    assert!(!path.is_file());
    path
}

fn random_id(ids: &'static [&'static str]) -> IdBuf {
    let i = rand::thread_rng()
        .gen_range(0..ids.len());

    Id::from_str(ids[i])
        .unwrap()
}

fn random_entry<T>(vec: &Vec<T>) -> &T {
    let i = rand::thread_rng()
        .gen_range(0..vec.len());
    &vec[i]
}

#[test]
fn fetcher_from_is_same() {
    let id = random_id(SIGNATURE_CIPHER);

    assert_eq!(
        VideoFetcher::from_url(&id.watch_url()).unwrap(),
        VideoFetcher::from_url(&id.embed_url()).unwrap(),
    );
    assert_eq!(
        VideoFetcher::from_url(&id.embed_url()).unwrap(),
        VideoFetcher::from_url(&id.share_url()).unwrap(),
    );
    assert_eq!(
        VideoFetcher::from_url(&id.share_url()).unwrap(),
        VideoFetcher::from_id(id).unwrap(),
    );
}

#[tokio::test]
#[ignore]
async fn video_from_is_same() {
    let id = random_id(SIGNATURE_CIPHER);

    assert_eq!(
        Video::from_url(&id.watch_url()).await.unwrap(),
        Video::from_id(id).await.unwrap()
    );
}

#[tokio::test]
#[ignore]
async fn fetch_and_descramble_is_same() {
    let id = random_id(SIGNATURE_CIPHER);

    let descrambler0 = VideoFetcher::from_id(id.as_owned())
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let descrambler1 = VideoFetcher::from_id(id)
        .unwrap()
        .fetch()
        .await
        .unwrap();

    assert_eq!(
        descrambler0,
        descrambler1
    );
    assert_eq!(
        descrambler0.clone().descramble().unwrap(),
        descrambler0.clone().descramble().unwrap()
    );
    assert_eq!(
        descrambler1.clone().descramble().unwrap(),
        descrambler1.clone().descramble().unwrap()
    );
    assert_eq!(
        descrambler0.descramble().unwrap(),
        descrambler1.descramble().unwrap()
    );
}

#[tokio::test]
#[ignore]
async fn download() {
    let id = random_id(SIGNATURE_CIPHER);
    let expected_path = download_path_from_id(id.as_borrowed()).await;

    let path: PathBuf = dbg!(Video::from_id(id.as_owned())
        .await
        .unwrap())
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

    let path = dbg!(Video::from_id(id)
        .await
        .unwrap())
        .worst_quality()
        .unwrap()
        .download_to_dir(DOWNLOAD_DIR)
        .await
        .unwrap();

    correct_path!(path, expected_path);
}

#[test]
#[ignore]
fn blocking_download_to_dir() {
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

    let path: PathBuf = dbg!(Video::from_id(id.as_owned())
        .await
        .unwrap())
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
    let path = download_path_from_id(id.as_borrowed()).await;

    let _: () = dbg!(Video::from_id(id)
        .await
        .unwrap())
        .worst_quality()
        .unwrap()
        .download_to(&path)
        .await
        .unwrap();

    correct_path!(&path, path);
}

#[tokio::test]
#[ignore]
async fn signature_cipher() {
    let id = random_id(SIGNATURE_CIPHER);

    let video = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
            .unwrap()
            .descramble()
            .unwrap()
    );

    assert!(random_entry(video.streams()).signature_cipher.s.is_some());
}

#[tokio::test]
#[ignore]
async fn pre_signed() {
    let id = random_id(PRE_SIGNED);

    let video = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
            .unwrap()
            .descramble()
            .unwrap()
    );

    assert!(random_entry(video.streams()).signature_cipher.s.is_none());
}

#[tokio::test]
#[ignore]
async fn age_restricted() {
    let id = random_id(AGE_RESTRICTED);

    let video = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
            .unwrap()
            .descramble()
            .unwrap()
    );

    assert!(video.is_age_restricted());
}

#[tokio::test]
#[ignore]
async fn private_video() {
    let id = random_id(PRIVATE);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match ps {
                PlayabilityStatus::LoginRequired { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", *ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[tokio::test]
#[ignore]
async fn region_blocked() {
    let id = random_id(REGION_BLOCKED);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match ps {
                PlayabilityStatus::LoginRequired { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[tokio::test]
#[ignore]
async fn missing_recording() {
    let id = random_id(MISSING_RECORDING);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match ps {
                PlayabilityStatus::Unplayable { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[tokio::test]
#[ignore]
async fn live_stream() {
    let id = random_id(LIVE_STREAM);

    let video = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
            .unwrap()
            .descramble()
            .unwrap()
    );

    assert!(video.video_details().is_live_content);
}
