use anyhow::Result;
use mime::Mime;
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

use rustube::Stream;

use crate::output_level::OutputLevel;

#[derive(Debug)]
pub struct StreamSerializer {
    pub output_level: OutputLevel,
    pub stream: Stream,
}

impl Serialize for StreamSerializer {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        macro_rules! serialize {
            ($self:ident, $map:ident; $($level:expr => { $($field:ident $(with $func:ident)?),* $(,)? })*) => {
                $(
                    if self.output_level.contains($level) {
                        $(
                            $map.serialize_entry(
                                stringify!($field),
                                &serialize!{ @__ser($self.stream.$field $(=> $func)?) }
                            )?;
                        )*
                    }
                )*
            };
            (@__ser($field:expr)) => { $field };
            (@__ser($field:expr => $func:ident)) => { $func(&$field) };
        }

        let mut map = serializer.serialize_map(None)?;

        serialize!(self, map;
            OutputLevel::URL => {
                signature_cipher 
            }
            
            OutputLevel::GENERAL => {
                mime with serialize_mime, quality, includes_video_track, includes_audio_track, 
                approx_duration_ms
            }
            OutputLevel::GENERAL | OutputLevel::VERBOSE => {
                codecs, is_progressive
            }
            
            OutputLevel::VIDEO_TRACK => {
                height, width, quality_label, fps
            }
            OutputLevel::VIDEO_TRACK | OutputLevel::VERBOSE => {
                format_type, color_info, high_replication, is_otf
            }
            
            OutputLevel::AUDIO_TRACK => {
                audio_quality, bitrate, audio_sample_rate, audio_channels, loudness_db
            }
            OutputLevel::AUDIO_TRACK | OutputLevel::VERBOSE => {
                average_bitrate
            }
            
            OutputLevel::all() => {
                index_range, init_range, itag, last_modified, projection_type
            }
        );

        map.end()
    }
}

fn serialize_mime(mime: &Mime) -> &str {
    mime.as_ref()
}
