use crate::config::game_config::*;
use crate::game_utils::{game_lib::*, game_map::*};
use crate::misc::utils::*;
use bevy::prelude::*;
use std::path::Path;

pub fn setup_game(
    args: Res<Args>,
    mut window: Single<&mut Window>,
    mut exit_app: MessageWriter<AppExit>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let Some(game_lib) = load_game_lib(
        args.config_path.as_path(),
        &mut exit_app,
        asset_server.as_ref(),
    ) else {
        return;
    };

    init_window(&game_lib.game_config, window.as_mut());
    commands.spawn(Camera2d);

    let mut game_obj_lib = GameObjLib::new();
    let mut screen_coord = ScreenCoord::new(&game_lib.game_config);

    let game_map_path = game_lib.game_config.map_dir().join(&args.map_path);
    let Some(game_map) = load_game_map(
        game_map_path,
        game_lib.game_config.cell_size,
        &game_lib,
        &mut game_obj_lib,
        &screen_coord,
        &mut commands,
        &mut exit_app,
    ) else {
        return;
    };

    commands.insert_resource(game_lib);
    commands.insert_resource(game_obj_lib);
    commands.insert_resource(screen_coord);
    commands.insert_resource(game_map);

    info!("Finished setup")
}

fn load_game_lib<P: AsRef<Path>>(
    config_path: P,
    exit_app: &mut MessageWriter<AppExit>,
    asset_server: &AssetServer,
) -> Option<GameLib> {
    let game_lib = match GameLib::load(config_path, asset_server) {
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
    game_lib: &GameLib,
    game_obj_lib: &mut GameObjLib,
    screen_coord: &ScreenCoord,
    commands: &mut Commands,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<GameMap> {
    let game_map = match GameMap::load(
        map_path,
        cell_size,
        game_lib,
        game_obj_lib,
        screen_coord,
        commands,
    ) {
        Ok(map) => map,
        Err(err) => {
            error!("Failed to load GameMap: {}", err);
            exit_app.write(AppExit::error());
            return None;
        }
    };

    Some(game_map)
}
