use crate::game_utils::WorldInfo;
use crate::misc::MyError;
use bevy::prelude::{Camera, GlobalTransform, Quat, Resource, Vec2, error};
use clap::{Parser, ValueEnum};
use serde::de::DeserializeOwned;
use serde_json;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, Clone, ValueEnum)]
pub enum AppMode {
    RunGame,
    GenMap,
}

#[derive(Parser, Resource)]
pub struct Args {
    #[arg(long, value_enum)]
    pub mode: AppMode,

    #[arg(long)]
    pub log: PathBuf,

    #[arg(long)]
    pub game_config: Option<PathBuf>,

    #[arg(long)]
    pub gen_map_config: Option<PathBuf>,

    #[arg(long)]
    pub map: Option<PathBuf>,
}

pub fn read_json<T, P>(path: P) -> Result<T, MyError>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;
    Ok(result)
}

pub fn setup_log<P: AsRef<Path>>(log_path: P) -> WorkerGuard {
    let log_file = File::create(log_path.as_ref()).expect("Open file");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(log_file);

    let file_layer = fmt::layer()
        .with_ansi(false) // Disable ANSI color codes for the file to keep it clean
        .with_writer(non_blocking_appender)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_thread_names(true);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(file_layer)
        .init();

    guard
}

#[inline]
pub fn arr_to_vec2(v: &[f32; 2]) -> Vec2 {
    Vec2 { x: v[0], y: v[1] }
}

#[inline]
pub fn get_rotation(d: &Vec2) -> Quat {
    let from = Vec2::new(1.0, 0.0);
    Quat::from_rotation_arc_2d(from, d.clone())
}

pub fn translate_cursor_pos(
    cursor_pos: Vec2,
    camera: &Camera,
    transform: &GlobalTransform,
    world_info: &WorldInfo,
) -> Option<Vec2> {
    let pos = match camera.viewport_to_world_2d(transform, cursor_pos) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to transform cursor position: {}", err);
            return None;
        }
    };

    Some(world_info.viewport_to_world(&pos))
}

#[macro_export]
macro_rules! log_and_get_err {
    ($format_str:expr) => {{
        let msg = format!($format_str);
        error!(msg);
        Err(MyError::Other(msg))
    }};

    ($format_str:expr, $($x:expr),+) => {{
        let msg = format!($format_str, $($x),+);
        error!(msg);
        Err(MyError::Other(msg))
    }};
}

#[macro_export]
macro_rules! obj_missing_from_lib {
    () => {
        $crate::log_and_get_err!("Failed to find obj in GameObjLib")
    };
}
