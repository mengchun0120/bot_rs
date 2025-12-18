use crate::game::game_map::*;
use crate::res::game_config::*;
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

    let Some(game_map) = load_game_map(
        args.map_path.as_path(),
        game_lib.game_config.cell_size,
        &mut exit_app,
    ) else {
        return;
    };

    commands.insert_resource(game_lib);
    commands.insert_resource(game_map);

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

fn load_game_map<P: AsRef<Path>>(
    map_path: P,
    cell_size: f32,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<GameMap> {
    let game_map = match GameMap::load(map_path, cell_size) {
        Ok(map) => map,
        Err(err) => {
            error!("Failed to load GameMap: {}", err);
            exit_app.write(AppExit::error());
            return None;
        }
    };

    Some(game_map)
}
