use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FramePlayConfig {
    pub frame_count: usize,
    pub frames_per_second: usize,
}

#[derive(Debug, Deserialize)]
pub struct PhaseoutConfig {
    pub duration: f32,
}

#[derive(Debug, Deserialize)]
pub enum PlayoutConfig {
    FramePlay(FramePlayConfig),
    Phaseout(PhaseoutConfig),
}
