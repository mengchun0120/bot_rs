use bevy::{log::LogPlugin, prelude::*};
use bot_rs::systems::*;
use bot_rs::utils::*;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let _guard = setup_log(&args.log_path);

    info!("log has been setup");

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(args)
        .add_systems(Startup, setup_game)
        .run();
}
