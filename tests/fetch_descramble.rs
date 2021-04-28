#![cfg(feature = "descramble")]

use common::*;
use rustube::VideoFetcher;

#[macro_use]
mod common;


#[test]
fn differently_constructed_fetchers_are_eq() {
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

#[test]
#[cfg(feature = "blocking")]
fn differently_constructed_blocking_fetchers_are_eq() {
    use rustube::blocking::VideoFetcher as BlockingVideoFetcher;

    let id = random_id(SIGNATURE_CIPHER);

    assert_eq!(
        BlockingVideoFetcher::from_url(&id.watch_url()).unwrap(),
        BlockingVideoFetcher::from_url(&id.embed_url()).unwrap(),
    );
    assert_eq!(
        BlockingVideoFetcher::from_url(&id.embed_url()).unwrap(),
        BlockingVideoFetcher::from_url(&id.share_url()).unwrap(),
    );
    assert_eq!(
        BlockingVideoFetcher::from_url(&id.share_url()).unwrap(),
        BlockingVideoFetcher::from_id(id).unwrap(),
    );
}

#[tokio::test]
#[ignore]
async fn two_video_descrambler_instances_of_same_video_are_eq() {
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
#[cfg(all(feature = "regex", feature = "download"))]
async fn differently_constructed_videos_are_eq() {
    use rustube::Video;

    let id = random_id(SIGNATURE_CIPHER);

    let video_from_url = Video::from_url(&id.watch_url()).await.unwrap();
    let video_from_fetcher = video!(id.as_owned());

    assert_eq!(
        Video::from_id(id).await.unwrap(),
        video_from_url
    );
    assert_eq!(
        video_from_url,
        video_from_fetcher
    );
}
