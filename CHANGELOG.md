## 0.5.0

### Changed

- The type of `Stream.last_modified` and `RawFormat.last_modified` changed from `DateTime<Utc>`
  to `Option<DateTime<Utc>>`

### Added

- New `LatencyClass` variant `UltraLow` (`MDE_STREAM_OPTIMIZATIONS_RENDERER_LATENCY_ULTRA_LOW`)

## 0.4.0

### Changed

- type of `ID_PATTERNS`, `WATCH_URL_PATTERN`, `SHORTS_URL_PATTERN`, `EMBED_URL_PATTERN`, `SHARE_URL_PATTERN`,
  `ID_PATTERN` in `crate::id`, and `RT` in `crate::blocking` to use `once_cell::sync::Lazy` instead of
  `std::lazy::Sync::Lazy`

### Removed

- the need for a nightly compiler

## 0.3.8

### Fixed

- file extension when downloading streams is no longer hardcoded to `mp4`, but depends on the stream mime instead

## 0.3.7

### Removed

- `VideoDetails.average_rating` due to API change

## 0.3.6

### Added

- support for `shorts` url (`youtube.com/shorts/<ID>`)

## 0.3.5

### Changed

- (internal) `VideoFetcher::get_video_info_and_js` now acquires the `VideoInfo` from the watch_html instead of `/get_video_info`

## 0.3.4

### Changed

- upgraded to newest dependencies

### Fixed

- `clippy::nonstandard-macro-braces` warning

## 0.3.3

### Added

- `QualityLabel::P144HDR` (144p HDR)
- `QualityLabel::P240HDR` (240p HDR)
- `QualityLabel::360HDR` (360p HDR)
- `QualityLabel::P480HDR` (480 HDR)
- `QualityLabel::P4320Hz60HDR` (4320p60 HDR)

### Changed

- made `PlayabilityStatus.miniplayer` optional
- made `PlayabilityStatus.miniplayer` optional
- made `PlayerResponse.microformat` optional
- (internal) applied a fix to `/get_video_info`

## 0.3.2

### Changed

- made the `embed` field of `PlayerMicroformatRenderer` `Option<_>`

## 0.2.3

### Added

A CLI. This CLI provides a small but useful subset of `rustube`. Currently available commands:

- `download`: downloads a video from YouTube
- `fetch`: fetches information about a video
- `check`: checks whether or not the video can be downloaded
