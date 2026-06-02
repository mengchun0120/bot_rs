use crate::config::{GameConfig, GameMapConfig};
use crate::game::create_obj_by_config;
use crate::game_utils::{
    DespawnPool, GameInfo, GameLib, GameMap, GameObjLib, NewObjQueue, WorldInfo,
};
use crate::misc::{Args, GameState, arr_to_vec2, read_json};
use bevy::prelude::*;

const PLAYER_CONFIG_NAME: &str = "player_bot";

pub fn setup_game(
    args: Res<Args>,
    game_lib: Res<GameLib>,
    mut commands: Commands,
    mut exit_app: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let game_config = &game_lib.game_config;
    let Some(map_config) = read_map_config(args.as_ref(), game_config, &mut exit_app) else {
        return;
    };
    let Some(mut world_info) = create_world_info(game_config, &map_config, &mut exit_app) else {
        return;
    };
    let mut game_obj_lib = GameObjLib::new();
    let mut game_info = GameInfo::new();

    let Some(game_map) = load_game_map(
        &map_config,
        game_config.cell_size,
        &mut world_info,
        &mut game_obj_lib,
        &game_lib,
        &mut commands,
        &mut exit_app,
        &mut game_info,
    ) else {
        return;
    };

    commands.insert_resource(game_map);
    commands.insert_resource(world_info);
    commands.insert_resource(game_obj_lib);
    commands.insert_resource(NewObjQueue::new());
    commands.insert_resource(DespawnPool::new());
    commands.insert_resource(game_info);

    game_state.set(GameState::Play);

    info!("Finished setup game")
}

fn read_map_config(
    args: &Args,
    game_config: &GameConfig,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<GameMapConfig> {
    let Some(map_path) = &args.map else {
        error!("map missing from args");
        return None;
    };
    let game_map_path = game_config.map_dir().join(map_path);
    let map_config: GameMapConfig = match read_json(game_map_path) {
        Ok(c) => c,
        Err(err) => {
            error!("Failed to read map from {:?}: {}", args.map, err);
            exit_app.write(AppExit::error());
            return None;
        }
    };
    Some(map_config)
}

fn create_world_info(
    game_config: &GameConfig,
    map_config: &GameMapConfig,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<WorldInfo> {
    let world_width = game_config.cell_size * map_config.col_count as f32;
    let world_height = game_config.cell_size * map_config.row_count as f32;
    let Some(player_pos) = find_player_pos(map_config) else {
        error!("Cannot find player in map");
        exit_app.write(AppExit::error());
        return None;
    };
    let world_info = WorldInfo::new(
        world_width,
        world_height,
        game_config.window_width(),
        game_config.window_height(),
        game_config.window_ext_size,
        &player_pos,
    );

    Some(world_info)
}

fn load_game_map(
    map_config: &GameMapConfig,
    cell_size: f32,
    world_info: &mut WorldInfo,
    game_obj_lib: &mut GameObjLib,
    game_lib: &GameLib,
    commands: &mut Commands,
    exit_app: &mut MessageWriter<AppExit>,
    game_info: &mut GameInfo,
) -> Option<GameMap> {
    let mut game_map = GameMap::new(map_config.row_count, map_config.col_count, cell_size);

    for map_obj_config in map_config.objs.iter() {
        if let Err(err) = create_obj_by_config(
            map_obj_config,
            world_info,
            &mut game_map,
            game_obj_lib,
            game_lib,
            commands,
            game_info,
        ) {
            error!("Failed to add obj: {}", err);
            exit_app.write(AppExit::error());
            return None;
        }
    }

    Some(game_map)
}

fn find_player_pos(map_config: &GameMapConfig) -> Option<Vec2> {
    for obj_config in map_config.objs.iter() {
        if obj_config.config_name == PLAYER_CONFIG_NAME {
            return Some(arr_to_vec2(&obj_config.pos));
        }
    }

    None
}
