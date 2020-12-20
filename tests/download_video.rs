#![feature(once_cell)]

use url::Url;

use rustube::{YouTube, YouTubeFetcher};

#[tokio::test]
async fn download_fetcher() {
    let url = Url::parse("https://www.youtube.com/watch?v=5jlI4uzZGjU&ab_channel=PitbullVEVO").unwrap();

    let descrambler = YouTubeFetcher::from_url(&url)
        .unwrap()
        .fetch()
        .await
        .unwrap();

    let yt = descrambler
        .descramble()
        .unwrap();

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
async fn download_youtube() {
    let url = Url::parse("https://www.youtube.com/watch?v=5jlI4uzZGjU&ab_channel=PitbullVEVO").unwrap();

    let yt = YouTube::from_url(&url)
        .await
        .unwrap();

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



