mod ai;
mod config;
mod game;
mod game_utils;
mod misc;
mod systems;

use crate::misc::*;
use crate::systems::*;
use bevy::{log::LogPlugin, prelude::*};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let _guard = setup_log(&args.log_path);

    info!("log has been setup");

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(args)
        .init_state::<AppState>()
        .add_systems(Startup, setup_app)
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}
