use crate::config::{GameObjConfig, NamedGameObjConfig};
use crate::misc::{MyError, check_collide_obj};
use crate::systems::gen_map::generated_map::GeneratedMap;
use bevy::prelude::*;
use core::f32;
use rand::{
    Rng, rng,
    rngs::ThreadRng,
    seq::{IndexedRandom, SliceRandom},
};
use std::fs::File;
use std::path::PathBuf;

pub fn gen_bots(
    map: &mut GeneratedMap,
    ai_bot_count: usize,
    player_config: &NamedGameObjConfig,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
) -> Result<(), MyError> {
    if ai_bot_configs.is_empty() {
        let msg = "ai_bot_configs is empty".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    }

    let max_size = get_max_bot_size(player_config, ai_bot_configs)?;

    info!("max_bot_size={}", max_size);

    let mut spots = get_candidate_spots_for_bots(map, max_size)?;
    if spots.len() == 0 {
        let msg = "Failed to generate bots: map is full".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    }

    info!("Candidate spots for bots: {}", spots.len());

    let mut r = rng();
    spots.shuffle(&mut r);

    let pos = spots.pop().unwrap();
    let direction = random_direction(&mut r);
    map.add(pos, direction, player_config.clone())?;

    let count = ai_bot_count.min(spots.len());
    for _ in 0..count {
        if let Some(pos) = spots.pop() {
            let direction = random_direction(&mut r);
            let bot_config = ai_bot_configs.choose(&mut r).unwrap();
            map.add(pos, direction, bot_config.clone())?;
        } else {
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

fn get_max_bot_size(
    player_config: &NamedGameObjConfig,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
) -> Result<f32, MyError> {
    let bot_config = player_config.bot_config()?;
    let mut max_size = bot_config.size[0].max(bot_config.size[1]);

    for config in ai_bot_configs.iter() {
        let bot_config = config.bot_config()?;
        let size = bot_config.size[0].max(bot_config.size[1]);

        if size > max_size {
            max_size = size;
        }
    }

    Ok(max_size)
}

fn get_candidate_spots_for_bots(
    map: &GeneratedMap,
    max_bot_size: f32,
) -> Result<Vec<Vec2>, MyError> {
    let mut spots: Vec<Vec2> = Vec::new();
    let half_size = max_bot_size / 2.0;
    let mut y = half_size;
    let height = map.height();
    let width = map.width();

    while y < height {
        let mut x = half_size;

        while x < width {
            let pos = Vec2::new(x, y);

            if !check_collide_tile(map, &pos, half_size)? {
                spots.push(pos);
            }

            x += max_bot_size;
        }

        y += max_bot_size;
    }

    Ok(spots)
}

fn check_collide_tile(map: &GeneratedMap, pos: &Vec2, collide_span: f32) -> Result<bool, MyError> {
    let region = map.get_map_region(
        pos.x - collide_span,
        pos.y - collide_span,
        pos.x + collide_span,
        pos.y + collide_span,
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

fn random_direction(r: &mut ThreadRng) -> Vec2 {
    let theta = r.random_range(0.0..(2.0 * f32::consts::PI));
    Vec2::new(theta.cos(), theta.sin())
}
