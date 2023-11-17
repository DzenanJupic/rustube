#![cfg(feature = "stream")]

use common::*;
use rustube::{Error, VideoFetcher};
use rustube::video_info::player_response::playability_status::PlayabilityStatus;

#[macro_use]
mod common;

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_has_signature_cipher() {
    let id = random_id(SIGNATURE_CIPHER);
    let video = video!(id);

    assert!(random_entry(video.streams()).signature_cipher.s.is_some());
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_is_pre_signed() {
    let id = random_id(PRE_SIGNED);
    let video = video!(id);

    assert!(random_entry(video.streams()).signature_cipher.s.is_none());
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_is_age_restricted() {
    let id = random_id(AGE_RESTRICTED);
    let video = video!(id);

    assert!(video.is_age_restricted());
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_is_private_video() {
    let id = random_id(PRIVATE);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match *ps {
                PlayabilityStatus::LoginRequired { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_is_region_blocked() {
    let id = random_id(REGION_BLOCKED);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match *ps {
                PlayabilityStatus::LoginRequired { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_has_missing_recording() {
    let id = random_id(MISSING_RECORDING);

    let res = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
    );

    match res.unwrap_err() {
        Error::VideoUnavailable(ps) => {
            match *ps {
                PlayabilityStatus::Unplayable { .. } => {}
                ps => panic!("expected LoginRequired, got: {:?}", ps)
            }
        }
        e => panic!("expected Error::VideoUnavailable, got: {:?}", e)
    }
}

#[test_env_log::test(tokio::test)]
#[ignore]
async fn video_is_live_stream() {
    let id = random_id(LIVE_STREAM);

    let video = dbg!(
        VideoFetcher::from_id(id)
            .unwrap()
            .fetch()
            .await
            .unwrap()
            .descramble()
            .await
            .unwrap()
    );

    assert!(video.video_details().is_live_content);
}
