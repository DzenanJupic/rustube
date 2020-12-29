const HDR: [u64; 8] = [330, 331, 332, 333, 334, 335, 336, 337];
const _60FPS: [u64; 14] = [298, 299, 302, 303, 308, 315, 330, 331, 332, 333, 334, 335, 336, 337];
const _3D: [u64; 7] = [82, 83, 84, 85, 100, 101, 102];
const LIVE: [u64; 8] = [91, 92, 93, 94, 95, 96, 132, 151];
const DASH_MP4_VIDEO: [u64; 12] = [133, 134, 135, 136, 137, 138, 160, 212, 264, 266, 298, 299];
const DASH_MP4_AUDIO: [u64; 7] = [139, 140, 141, 256, 258, 325, 328];
const DASH_WEBM_VIDEO: [u64; 21] = [167, 168, 169, 170, 218, 219, 278, 242, 243, 244, 245, 246, 247, 248, 271, 272, 302, 303, 308, 313, 315];
const DASH_WEBM_AUDIO: [u64; 5] = [171, 172, 249, 250, 251];

pub struct ItagProfile {
    pub resolution: Option<u64>,
    pub abr: Option<u64>,
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

/// return the resolution in pixels and the abr in kbs
const fn resolution_abr_from_itag(itag: u64) -> (Option<u64>, Option<u64>) {
    match itag {
        5 => (Some(240), Some(64)),
        6 => (Some(270), Some(64)),
        13 => (Some(144), None),
        17 => (Some(144), Some(24)),
        18 => (Some(360), Some(96)),
        22 => (Some(720), Some(192)),
        34 => (Some(360), Some(128)),
        35 => (Some(480), Some(128)),
        36 => (Some(240), None),
        37 => (Some(1080), Some(192)),
        38 => (Some(3072), Some(192)),
        43 => (Some(360), Some(128)),
        44 => (Some(480), Some(128)),
        45 => (Some(720), Some(192)),
        46 => (Some(1080), Some(192)),
        59 => (Some(480), Some(128)),
        78 => (Some(480), Some(128)),
        82 => (Some(360), Some(128)),
        83 => (Some(480), Some(128)),
        84 => (Some(720), Some(192)),
        85 => (Some(1080), Some(192)),
        91 => (Some(144), Some(48)),
        92 => (Some(240), Some(48)),
        93 => (Some(360), Some(128)),
        94 => (Some(480), Some(128)),
        95 => (Some(720), Some(256)),
        96 => (Some(1080), Some(256)),
        100 => (Some(360), Some(128)),
        101 => (Some(480), Some(192)),
        102 => (Some(720), Some(192)),
        132 => (Some(240), Some(48)),
        151 => (Some(720), Some(24)),
        // DASH Video
        133 => (Some(240), None),
        134 => (Some(360), None),
        135 => (Some(480), None),
        136 => (Some(720), None),
        137 => (Some(1080), None),
        138 => (Some(2160), None),
        160 => (Some(144), None),
        167 => (Some(360), None),
        168 => (Some(480), None),
        169 => (Some(720), None),
        170 => (Some(1080), None),
        212 => (Some(480), None),
        218 => (Some(480), None),
        219 => (Some(480), None),
        242 => (Some(240), None),
        243 => (Some(360), None),
        244 => (Some(480), None),
        245 => (Some(480), None),
        246 => (Some(480), None),
        247 => (Some(720), None),
        248 => (Some(1080), None),
        264 => (Some(1440), None),
        266 => (Some(2160), None),
        271 => (Some(1440), None),
        272 => (Some(2160), None),
        278 => (Some(144), None),
        298 => (Some(720), None),
        299 => (Some(1080), None),
        302 => (Some(720), None),
        303 => (Some(1080), None),
        308 => (Some(1440), None),
        313 => (Some(2160), None),
        315 => (Some(2160), None),
        330 => (Some(144), None),
        331 => (Some(240), None),
        332 => (Some(360), None),
        333 => (Some(480), None),
        334 => (Some(720), None),
        335 => (Some(1080), None),
        336 => (Some(1440), None),
        337 => (Some(2160), None),
        // DASH Audio
        139 => (None, Some(48)),
        140 => (None, Some(128)),
        141 => (None, Some(256)),
        171 => (None, Some(128)),
        172 => (None, Some(256)),
        249 => (None, Some(50)),
        250 => (None, Some(70)),
        251 => (None, Some(160)),

        _ => (None, None)
    }
}
