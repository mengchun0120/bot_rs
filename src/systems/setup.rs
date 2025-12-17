use crate::res::config::*;
use crate::res::game_lib::*;
use crate::utils::*;
use bevy::prelude::*;
use std::path::Path;

pub fn setup_game(
    args: Res<Args>,
    mut window: Single<&mut Window>,
    mut exit_app: MessageWriter<AppExit>,
    mut commands: Commands,
) {
    let Some(game_lib) = load_game_lib(args.config_path.as_path(), &mut exit_app) else {
        return;
    };

    init_window(&game_lib.game_config, window.as_mut());

    commands.insert_resource(game_lib);

    info!("Finished setup")
}

fn load_game_lib<P: AsRef<Path>>(
    config_path: P,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<GameLib> {
    let game_lib = match GameLib::load(config_path) {
        Ok(lib) => lib,
        Err(err) => {
            error!("Failed to initialize GameLib: {}", err);
            exit_app.write(AppExit::error());
            return None;
        }
    };

    Some(game_lib)
}

fn init_window(game_config: &GameConfig, window: &mut Window) {
    window
        .resolution
        .set(game_config.window_width(), game_config.window_height());
}
