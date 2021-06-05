use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

use rustube::{Stream, VideoInfo};

use crate::output_level::OutputLevel;
use crate::stream_serializer::StreamSerializer;

#[derive(Debug)]
pub struct VideoSerializer {
    output_level: OutputLevel,
    video_info: VideoInfo,
    streams: Vec<StreamSerializer>,
}

impl VideoSerializer {
    pub fn new(video_info: VideoInfo, streams: impl Iterator<Item=Stream>, output_level: OutputLevel) -> Self {
        let streams = streams
            .map(|stream| StreamSerializer { stream, output_level })
            .collect::<Vec<_>>();

        Self {
            output_level,
            video_info,
            streams,
        }
    }
}

impl Serialize for VideoSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut map = serializer.serialize_map(None)?;

        if self.output_level.contains(OutputLevel::VIDEO) {
            map.serialize_entry("video_info", &self.video_info)?;
        }
        map.serialize_entry("streams", &self.streams)?;

        map.end()
    }
}
