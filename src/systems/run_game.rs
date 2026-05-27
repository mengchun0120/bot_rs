use crate::misc::{AppState, Args};
use crate::systems::{game_plugin, menu_plugin, setup_app, splash_plugin};
use bevy::{log::LogPlugin, prelude::*};

pub fn run_game(args: Args) {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(args)
        .init_state::<AppState>()
        .add_systems(Startup, setup_app)
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}
