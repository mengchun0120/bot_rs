use crate::config::{
    GameConfig, GameObjConfig, GameObjSide, GenMapAlgorithmConfig, GenMapConfig, NamedGameObjConfig,
};
use crate::misc::{Args, read_json};
use crate::systems::gen_map::gen_island_map::gen_island_map;
use bevy::prelude::*;
use std::path::PathBuf;

pub fn gen_map(args: Args) {
    let Some((game_config_path, gen_map_config_path, map_path)) = validate_args(&args) else {
        return;
    };
    let Some((game_config, game_obj_configs, gen_map_config)) =
        load_configs(game_config_path, gen_map_config_path)
    else {
        return;
    };
    let Some((player_config, ai_bot_configs, tile_configs)) =
        extract_obj_configs(&game_obj_configs)
    else {
        return;
    };

    let Ok(map) = (match &gen_map_config.algorithm {
        GenMapAlgorithmConfig::Island(_) => gen_island_map(
            &game_config,
            &gen_map_config,
            &player_config,
            &ai_bot_configs,
            &tile_configs,
        ),
        _ => {
            error!("Unsupported algorithm for generating map");
            return;
        }
    }) else {
        return;
    };
}

fn validate_args(args: &Args) -> Option<(&PathBuf, &PathBuf, &PathBuf)> {
    let Some(game_config_path) = &args.game_config else {
        error!("game_config missing from Args");
        return None;
    };
    let Some(gen_map_config_path) = &args.gen_map_config else {
        error!("gen_map_config missing from Args");
        return None;
    };
    let Some(map_path) = &args.map else {
        error!("map missing from Args");
        return None;
    };

    Some((game_config_path, gen_map_config_path, map_path))
}

fn load_configs(
    game_config_path: &PathBuf,
    gen_map_config_path: &PathBuf,
) -> Option<(GameConfig, Vec<NamedGameObjConfig>, GenMapConfig)> {
    let game_config: GameConfig = match read_json(game_config_path) {
        Ok(config) => config,
        Err(err) => {
            error!(
                "Failed to read GameConfig from {:?}: {}",
                game_config_path.as_os_str(),
                err
            );
            return None;
        }
    };

    let game_obj_config_path = game_config
        .config_dir()
        .join(game_config.game_obj_config_file());
    let game_obj_configs: Vec<NamedGameObjConfig> = match read_json(&game_obj_config_path) {
        Ok(obj_configs) => obj_configs,
        Err(err) => {
            error!(
                "Failed to read GameObjConfig's from {:?}: {}",
                game_obj_config_path.as_os_str(),
                err
            );
            return None;
        }
    };

    let gen_map_config: GenMapConfig = match read_json(gen_map_config_path) {
        Ok(config) => config,
        Err(err) => {
            error!(
                "Failed to read GenMapConfig from {:?}: {}",
                gen_map_config_path.as_os_str(),
                err
            );
            return None;
        }
    };

    Some((game_config, game_obj_configs, gen_map_config))
}

fn extract_obj_configs(
    obj_configs: &Vec<NamedGameObjConfig>,
) -> Option<(
    NamedGameObjConfig,
    Vec<NamedGameObjConfig>,
    Vec<NamedGameObjConfig>,
)> {
    let mut player_config: Option<NamedGameObjConfig> = None;
    let mut ai_bot_configs: Vec<NamedGameObjConfig> = Vec::new();
    let mut tile_configs: Vec<NamedGameObjConfig> = Vec::new();

    for obj_config in obj_configs.iter() {
        match &obj_config.config {
            GameObjConfig::Bot(bot_config) => match bot_config.side {
                GameObjSide::Player => {
                    player_config = Some(obj_config.clone());
                }
                GameObjSide::Ai => {
                    ai_bot_configs.push(obj_config.clone());
                }
                _ => {}
            },
            GameObjConfig::Tile(tile_config) => {
                tile_configs.push(obj_config.clone());
            }
            _ => {}
        }
    }

    let player_config = match player_config {
        Some(config) => config,
        None => {
            error!("Cannot find player config");
            return None;
        }
    };

    if ai_bot_configs.is_empty() {
        error!("Cannot find configs for AI bots");
        return None;
    }

    if tile_configs.is_empty() {
        error!("Cannot find configs for tiles");
        return None;
    }

    Some((player_config, ai_bot_configs, tile_configs))
}
