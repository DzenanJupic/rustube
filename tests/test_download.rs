#![feature(once_cell)]

use url::Url;

use rustube::{YouTube, YouTubeFetcher};

#[tokio::test]
async fn download_fetcher() {
    let url = Url::parse("https://www.youtube.com/watch?v=SmM0653YvXU&ab_channel=PitbullVEVO").unwrap();

    let descrambler = YouTubeFetcher::from_url(&url)
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let yt = dbg!(descrambler
        .descramble()
        .unwrap());

    let stream = yt
        .streams()
        .iter()
        .filter(|s| s.mime.subtype() == "mp4")
        .next()
        .unwrap();

    let path = stream
        .download()
        .await
        .unwrap();

    dbg!(path);
}

#[tokio::test]
async fn download_best_resolution() {
    let url = Url::parse("https://www.youtube.com/watch?v=5jlI4uzZGjU&ab_channel=PitbullVEVO").unwrap();

    let path = dbg!(YouTube::from_url(&url)
        .await.unwrap())
        .download_best_resolution()
        .await.unwrap();

    dbg!(path);
}

#[tokio::test]
async fn download_youtube_chain() {
    let url = Url::parse("https://www.youtube.com/watch?v=5jlI4uzZGjU&ab_channel=PitbullVEVO").unwrap();

    let path = dbg!(YouTube::from_url(&url)
        .await
        .unwrap())
        .streams()
        .iter()
        .filter(|s| s.mime.subtype() == "mp4")
        .next()
        .unwrap()
        .download()
        .await
        .unwrap();

    dbg!(path);
}



