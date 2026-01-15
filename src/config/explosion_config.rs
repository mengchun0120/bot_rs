use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Resource, Deserialize)]
pub struct ExplosionConfig {
    pub damage: f32,
    pub explode_span: f32,
    pub image: String,
    pub size: [u32; 2],
    pub frame_count: u32,
    pub frames_per_second: usize,
    pub z: f32,
}