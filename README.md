# <div align="center"> rustube </div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/rustube?style=flat-square)](https://crates.io/crates/rustube)
[![docs.rs](https://img.shields.io/docsrs/rustube?style=flat-square)](https://docs.rs/rustube)
[![licence](https://img.shields.io/crates/l/rustube?style=flat-square)](https://github.com/DzenanJupic/rustube)

![build](https://img.shields.io/github/workflow/status/DzenanJupic/rustube/Check%20and%20Build%20code%20on%20all%20major%20operating%20systems?style=flat-square)
![YT download](https://img.shields.io/github/workflow/status/DzenanJupic/rustube/Test%20downloading?label=YT%20download&style=flat-square)

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
#[tokio::main]
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

To install it, simply run

```
cargo +nightly install rustube-cli
```

After you successfully installed `rustube-cli`, you have access to the command `rustube`

```
A simple CLI for the rustube YouTube-downloader library.
For documentation and more information about rustube or the rustube-cli checkout
`https://github.com/DzenanJupic/rustube`.

For help with certain subcommands run `rustube <SUBCOMMAND> --help`.

USAGE:
    rustube <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    check       Checks if a video can be downloaded and fetches information about it
                This command is similar to fetch, in the way that it also fetches information
                about a video, but, other then fetch, will also decrypt all stream URLs.
                Therefore you can use the returned URLs for downloading the video. This of
                course means that the video has to be downloadable.
                By default this command will check for any streams that contain a video and an
                audio track. To specify other behavior, like checking for a stream with a
                particular quality, have a look at the subcommand help.
    download    Downloads a YouTube video
                By default, the Stream with the best quality and both a video, and an audio
                track will be downloaded. To specify other download behavior, have a look the
                the subcommand help.
    fetch       Fetches information about a YouTube video
                Fetching information does not require the video to actually be downloadable. So
                this also works when a video is, i.e., an offline stream. The downside is that
                you often don't have access to the stream URLs. Some videos come with pre-
                decrypted urls, in which case you can also use these to download the video, but
                if the video url is encrypted there's no way for you to download the video using
                only the returned information. To get decrypted URLs, have a look at `check`.
                For most use cases it's recommended to use `check` instead, since it gives you
                both more control and more information.
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
