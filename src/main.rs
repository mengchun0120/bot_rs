use bevy::{log::LogPlugin, prelude::*};
use bot_rs::misc::utils::*;
use bot_rs::systems::*;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let _guard = setup_log(&args.log_path);

    info!("log has been setup");

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(args)
        .add_systems(Startup, setup_game)
        .add_systems(PreUpdate, update_ai)
        .add_systems(
            Update,
            (
                process_cursor,
                process_mouse_button,
                process_key,
                update_player,
                update_ai_bots,
                update_missiles,
                update_explosions,
            ),
        )
        .add_systems(PostUpdate, (cleanup, add_new_objs))
        .run();
}
