use crate::config::{GameObjConfig, NamedGameObjConfig};
use crate::misc::{MyError, check_collide_obj};
use crate::systems::gen_map::generated_map::GeneratedMap;
use bevy::prelude::*;
use core::f32;
use rand::{Rng, rng, rngs::ThreadRng, seq::SliceRandom};
use std::fs::File;
use std::path::PathBuf;

pub fn gen_bots(
    map: &mut GeneratedMap,
    ai_bot_count: usize,
    player_config: &NamedGameObjConfig,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
) -> Result<(), MyError> {
    let max_bot_collide_span = get_max_bot_collide_span(player_config, ai_bot_configs)?;
    let mut spots = get_candidate_spots_for_bots(map.width(), map.height(), max_bot_collide_span);
    if spots.len() == 0 {
        let msg = "Failed to generate bots: map is full".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    }

    let mut r = rng();
    spots.shuffle(&mut r);

    if !add_bot_to_map(map, &mut spots, player_config, max_bot_collide_span, &mut r)? {
        let msg = "Failed to add player to map".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    }

    for _ in 0..ai_bot_count {
        if !add_random_bot_to_map(
            map,
            &mut spots,
            ai_bot_configs,
            max_bot_collide_span,
            &mut r,
        )? {
            break;
        }
    }

    Ok(())
}

pub fn write_gen_map(map: &GeneratedMap, file_path: &PathBuf) -> bool {
    let map_config = map.to_map_config();
    let file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            error!("Faild to write to map {:?}: {}", file_path, err);
            return false;
        }
    };

    if let Err(err) = serde_json::to_writer_pretty(file, &map_config) {
        error!("Failed to write to map {:?}: {}", file_path, err);
        return false;
    }

    true
}

fn get_max_bot_collide_span(
    player_config: &NamedGameObjConfig,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
) -> Result<f32, MyError> {
    let bot_config = player_config.bot_config()?;
    let mut max_collide_span = bot_config.collide_span;

    for config in ai_bot_configs.iter() {
        let bot_config = config.bot_config()?;

        if bot_config.collide_span > max_collide_span {
            max_collide_span = bot_config.collide_span;
        }
    }

    Ok(max_collide_span)
}

fn get_candidate_spots_for_bots(width: f32, height: f32, max_bot_collide_span: f32) -> Vec<Vec2> {
    let mut spots: Vec<Vec2> = Vec::new();
    let span = 2.0 * max_bot_collide_span;
    let mut y = max_bot_collide_span;

    while y < height {
        let mut x = max_bot_collide_span;

        while x < width {
            spots.push(Vec2::new(x, y));
            x += span;
        }

        y += span;
    }

    spots
}

fn check_collide_tile(
    map: &GeneratedMap,
    pos: &Vec2,
    collide_span: f32,
    max_bot_collide_span: f32,
) -> Result<bool, MyError> {
    let region = map.get_map_region(
        pos.x - max_bot_collide_span,
        pos.y - max_bot_collide_span,
        pos.x + max_bot_collide_span,
        pos.y + max_bot_collide_span,
    )?;

    for row in region.start_row..=region.end_row {
        for col in region.start_col..=region.end_col {
            for item in map.get_cell(row, col).iter() {
                let GameObjConfig::Tile(tile_config) = &(item.config.config) else {
                    continue;
                };

                if check_collide_obj(pos, collide_span, &item.pos, tile_config.collide_span) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

fn add_bot_to_map(
    map: &mut GeneratedMap,
    spots: &mut Vec<Vec2>,
    obj_config: &NamedGameObjConfig,
    max_bot_collide_span: f32,
    r: &mut ThreadRng,
) -> Result<bool, MyError> {
    let collide_span = obj_config.bot_config()?.collide_span;

    for i in 0..spots.len() {
        let collide = check_collide_tile(map, &spots[i], collide_span, max_bot_collide_span)?;
        if !collide {
            let theta = r.random_range(0.0..(2.0 * f32::consts::PI));
            let direction = Vec2::new(theta.cos(), theta.sin());

            map.add(spots[i].clone(), direction, obj_config.clone())?;
            spots.swap_remove(i);

            return Ok(true)
        }
    }

    Ok(false)
}

fn add_random_bot_to_map(
    map: &mut GeneratedMap,
    spots: &mut Vec<Vec2>,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
    max_bot_collide_span: f32,
    r: &mut ThreadRng,
) -> Result<bool, MyError> {
    let mut ai_bot_indices: Vec<usize> = (0..ai_bot_configs.len()).collect();
    ai_bot_indices.shuffle(r);

    for i in ai_bot_indices {
        if add_bot_to_map(map, spots, &ai_bot_configs[i], max_bot_collide_span, r)? {
            return Ok(true);
        }
    }

    Ok(false)
}
