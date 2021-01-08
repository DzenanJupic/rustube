# rustube
A complete (WIP), and easy to use YouTube downloader.
`rustube` is still in development and may see breaking changes!

- [API documentation]

### Table of content

- [Roadmap](#roadmap)
- [Usage](#usage)

## Roadmap

- [x] download normal videos
- [x] download age restricted videos
- [x] asynchronous API
- [x] blocking API
- [ ] full video info deserialization
- [ ] feature based opt-in deserialization of video info
- [ ] CLI
- [x] testing suite
- [ ] benchmarks
- [ ] Python bindings
- [ ] C / C++ bindings

## Usage

`rustube` provides an extremely easy to use API. If you just want to download a video and don't care about any
intermediate steps and any video information, this is all you need:

```rust
#[rustube::tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor";
    println!("downloaded video to {:?}", rustube::download_best_quality(&url).await.unwrap());
}
```

Of course, `rustube` has a lot more capabilities than just downloading videos. It also aims at deserializing the
complete video information provided by YouTube (this is still WIP). So finding out the view count of a video is as
simple as:

```rust
use rustube::{Id, VideoFetcher};

let id = Id::from_raw("https://www.youtube.com/watch?v=Edx9D2yaOGs&ab_channel=CollegeHumor").unwrap();
let descrambler = VideoFetcher::from_id(id.into_owned())
        .unwrap()
        .fetch()
        .await
        .unwrap();

let view_count = descrambler.video_details().view_count;
let title = descrambler.video_title();
println!("The video `{}` was viewed {} times.", title, view_count);
```

For more examples, an overview of the blocking API have a look at the [API documentation].


[API documentation]: https://docs.rs/rustube/
