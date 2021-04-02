# <div style="text-align: center;"> rustube </div>

<div style="text-align: center;">

[![crates.io](https://img.shields.io/crates/v/rustube)](https://crates.io/crates/rustube)
[![docs.rs](https://docs.rs/rustube/badge.svg)](https://docs.rs/rustube)
[![licence](https://img.shields.io/crates/l/rustube)](https://github.com/DzenanJupic/rustube)

</div>

---

A complete (WIP), and easy to use YouTube downloader.
> **Note**: `rustube` is still in development and may see breaking changes! It currently still requires a nightly
> compiler. This will change over time.

## Overview

- [Roadmap](#roadmap)
- [Usage](#usage)
- [CLI](#cli)
- [Contributing](#contributing)

## Roadmap

- [x] download normal videos
- [x] download age restricted videos
- [x] asynchronous API
- [x] blocking API
- [ ] full video info deserialization
- [ ] feature based opt-in deserialization of video info
- [x] CLI
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

## CLI

> **Note**: Currently, `rustube-cli` still requires a nightly compiler. This, as well as the commands, the flags, and
> the output, will likely change over time.

`rustube` comes with a CLI, `rustube-cli`, so you can download your favorite YouTube videos without having to write a
single line of code.

To install it, simply run run

```
cargo +nightly install rustube-cli
```

After you successfully installed `rustube-cli`, you have access to the command `rustube`

```
> rustube
rustube-cli

USAGE:
    rustube <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check       Checks whether or not a video can be downloaded and if so, prints all available
                streams
                The check includes fetching, parsing, and descrambling the video data, and also
                ensuring there is at least one Stream
                Since the video information gets descrambled, you can use all Stream URLs to
                access the video online
    download    Downloads a YouTube video
                By default, the Stream with the best quality and both a video, and an audio
                track will be downloaded
    fetch       Fetches information about a video, and prints it
                Contrary to the name, this will actually fetch and descramble the video
                information, so you can directly use all Stream URLs to access the video online
    help        Prints this message or the help of the given subcommand(s)
```

## Contributing

`rustube` is still in a pretty early stage, and you are welcome to contribute to it! The goal is to utilize the speed,
the type system, and the safety of Rust to make the fastest, most reliable, and most complete YouTube downloader out
there.

This project is 100% open source. Any contribution submitted for inclusion in `rustube` by you, shall have both the MIT
licence and the Apache-2.0 licence, and shall be licensed as MIT OR Apache-2.0, without any additional terms or
conditions.

### Licence

This project is licensed under the terms of the MIT licence or the Apache-2.0 licence, at your own choice.


[API documentation]: https://docs.rs/rustube/
