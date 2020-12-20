const HDR: [u64; 8] = [330, 331, 332, 333, 334, 335, 336, 337];
const _60FPS: [u64; 14] = [298, 299, 302, 303, 308, 315, 330, 331, 332, 333, 334, 335, 336, 337];
const _3D: [u64; 7] = [82, 83, 84, 85, 100, 101, 102];
const LIVE: [u64; 8] = [91, 92, 93, 94, 95, 96, 132, 151];
const DASH_MP4_VIDEO: [u64; 12] = [133, 134, 135, 136, 137, 138, 160, 212, 264, 266, 298, 299];
const DASH_MP4_AUDIO: [u64; 7] = [139, 140, 141, 256, 258, 325, 328];
const DASH_WEBM_VIDEO: [u64; 21] = [167, 168, 169, 170, 218, 219, 278, 242, 243, 244, 245, 246, 247, 248, 271, 272, 302, 303, 308, 313, 315];
const DASH_WEBM_AUDIO: [u64; 5] = [171, 172, 249, 250, 251];

pub struct ItagProfile {
    pub resolution: Option<&'static str>,
    pub abr: Option<&'static str>,
    pub is_live: bool,
    pub is_3d: bool,
    pub is_hdr: bool,
    pub fps: u8,
    pub is_dash: bool,
}

impl ItagProfile {
    pub fn from_itag(itag: u64) -> Self {
        let (resolution, abr) = resolution_abr_from_itag(itag);

        ItagProfile {
            resolution,
            abr,
            is_live: LIVE.contains(&itag),
            is_3d: _3D.contains(&itag),
            is_hdr: HDR.contains(&itag),
            // todo: this is probably false. There's a fps field in PlayerResponse
            fps: _60FPS.contains(&itag).then_some(60).unwrap_or(30),
            is_dash: DASH_MP4_VIDEO.contains(&itag) || DASH_MP4_AUDIO.contains(&itag) || DASH_WEBM_VIDEO.contains(&itag) || DASH_WEBM_AUDIO.contains(&itag),
        }
    }
}

const fn resolution_abr_from_itag(itag: u64) -> (Option<&'static str>, Option<&'static str>) {
    match itag {
        5 => (Some("240p"), Some("64kbps")),
        6 => (Some("270p"), Some("64kbps")),
        13 => (Some("144p"), None),
        17 => (Some("144p"), Some("24kbps")),
        18 => (Some("360p"), Some("96kbps")),
        22 => (Some("720p"), Some("192kbps")),
        34 => (Some("360p"), Some("128kbps")),
        35 => (Some("480p"), Some("128kbps")),
        36 => (Some("240p"), None),
        37 => (Some("1080p"), Some("192kbps")),
        38 => (Some("3072p"), Some("192kbps")),
        43 => (Some("360p"), Some("128kbps")),
        44 => (Some("480p"), Some("128kbps")),
        45 => (Some("720p"), Some("192kbps")),
        46 => (Some("1080p"), Some("192kbps")),
        59 => (Some("480p"), Some("128kbps")),
        78 => (Some("480p"), Some("128kbps")),
        82 => (Some("360p"), Some("128kbps")),
        83 => (Some("480p"), Some("128kbps")),
        84 => (Some("720p"), Some("192kbps")),
        85 => (Some("1080p"), Some("192kbps")),
        91 => (Some("144p"), Some("48kbps")),
        92 => (Some("240p"), Some("48kbps")),
        93 => (Some("360p"), Some("128kbps")),
        94 => (Some("480p"), Some("128kbps")),
        95 => (Some("720p"), Some("256kbps")),
        96 => (Some("1080p"), Some("256kbps")),
        100 => (Some("360p"), Some("128kbps")),
        101 => (Some("480p"), Some("192kbps")),
        102 => (Some("720p"), Some("192kbps")),
        132 => (Some("240p"), Some("48kbps")),
        151 => (Some("720p"), Some("24kbps")),
        // DASH Video
        133 => (Some("240p"), None),
        134 => (Some("360p"), None),
        135 => (Some("480p"), None),
        136 => (Some("720p"), None),
        137 => (Some("1080p"), None),
        138 => (Some("2160p"), None),
        160 => (Some("144p"), None),
        167 => (Some("360p"), None),
        168 => (Some("480p"), None),
        169 => (Some("720p"), None),
        170 => (Some("1080p"), None),
        212 => (Some("480p"), None),
        218 => (Some("480p"), None),
        219 => (Some("480p"), None),
        242 => (Some("240p"), None),
        243 => (Some("360p"), None),
        244 => (Some("480p"), None),
        245 => (Some("480p"), None),
        246 => (Some("480p"), None),
        247 => (Some("720p"), None),
        248 => (Some("1080p"), None),
        264 => (Some("1440p"), None),
        266 => (Some("2160p"), None),
        271 => (Some("1440p"), None),
        272 => (Some("2160p"), None),
        278 => (Some("144p"), None),
        298 => (Some("720p"), None),
        299 => (Some("1080p"), None),
        302 => (Some("720p"), None),
        303 => (Some("1080p"), None),
        308 => (Some("1440p"), None),
        313 => (Some("2160p"), None),
        315 => (Some("2160p"), None),
        330 => (Some("144p"), None),
        331 => (Some("240p"), None),
        332 => (Some("360p"), None),
        333 => (Some("480p"), None),
        334 => (Some("720p"), None),
        335 => (Some("1080p"), None),
        336 => (Some("1440p"), None),
        337 => (Some("2160p"), None),
        // DASH Audio
        139 => (None, Some("48kbps")),
        140 => (None, Some("128kbps")),
        141 => (None, Some("256kbps")),
        171 => (None, Some("128kbps")),
        172 => (None, Some("256kbps")),
        249 => (None, Some("50kbps")),
        250 => (None, Some("70kbps")),
        251 => (None, Some("160kbps")),

        _ => (None, None)
    }
}
