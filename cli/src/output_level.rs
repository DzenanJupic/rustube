use std::str::FromStr;

bitflags::bitflags! {
    pub struct OutputLevel: u8 {
        const URL           = 0b00000001;
        const GENERAL       = 0b00000010;
        const VIDEO_TRACK   = 0b00000100;
        const AUDIO_TRACK   = 0b00001000;
        const VERBOSE       = 0b10000000;
        
        const VIDEO         = 0b01000000;
    }
}

impl FromStr for OutputLevel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(|c: char| c.is_whitespace() || c == ',' || c == '|');
        let mut level = Self::empty();

        for s in split {
            if s.is_empty() { continue; }

            let next_level = match s {
                "url" => Self::URL,
                "general" => Self::GENERAL,
                "video-track" => Self::VIDEO_TRACK,
                "audio-track" => Self::AUDIO_TRACK,
                "verbose" => Self::VERBOSE,
                "full" => Self::all(),

                "video" => Self::VIDEO,

                _ => anyhow::bail!("could not parse {:?} to an OutputLevel", s)
            };
            level |= next_level;
        }

        Ok(level)
    }
} 
