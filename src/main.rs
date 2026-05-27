mod ai;
mod config;
mod game;
mod game_utils;
mod misc;
mod systems;

use crate::misc::{AppMode, Args, setup_log};
use crate::systems::{gen_map, run_game};
use bevy::prelude::*;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let _guard = setup_log(&args.log);

    info!("log has been setup");

    match args.mode {
        AppMode::RunGame => run_game(args),
        AppMode::GenMap => gen_map(args),
    }
}
