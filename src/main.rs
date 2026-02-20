use bevy::{log::LogPlugin, prelude::*};
use bot_rs::misc::{AppState, Args, setup_log};
use bot_rs::systems::{menu_plugin, setup_app, splash_plugin};
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
        .add_plugins((splash_plugin, menu_plugin))
        .run();
}
