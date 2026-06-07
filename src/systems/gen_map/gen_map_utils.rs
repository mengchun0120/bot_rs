use crate::{
    config::{GameObjConfig, NamedGameObjConfig},
    misc::{MyError, check_collide_obj},
    systems::gen_map::generated_map::{GeneratedMap, GeneratedMapItem},
};
use bevy::prelude::*;
use rand::{rng, seq::SliceRandom};

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



    for _ in 0..ai_bot_count {

    }

    Ok(())
}

fn get_max_tile_collide_span(tile_configs: &Vec<NamedGameObjConfig>) -> Result<f32, MyError> {
    let mut max_collide_span = 0.0;

    for config in tile_configs.iter() {
        let tile_config = config.tile_config()?;

        if tile_config.collide_span > max_collide_span {
            max_collide_span = tile_config.collide_span;
        }
    }

    Ok(max_collide_span)
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

fn get_candidate_spots_for_bots(
    width: f32,
    height: f32,
    max_bot_collide_span: f32,
) -> Vec<(f32, f32)> {
    let mut spots: Vec<(f32, f32)> = Vec::new();
    let span = 2.0 * max_bot_collide_span;
    let mut y = max_bot_collide_span;

    while y < height {
        let mut x = max_bot_collide_span;

        while x < width {
            spots.push((x, y));
            x += span;
        }

        y += span;
    }

    spots
}

fn check_collide_tile(
    map: &GeneratedMap,
    x: f32,
    y: f32,
    collide_span: f32,
    max_bot_collide_span: f32,
) -> Result<bool, MyError> {
    let region = map.get_map_region(
        x - max_bot_collide_span,
        y - max_bot_collide_span,
        x + max_bot_collide_span,
        y + max_bot_collide_span,
    )?;
    let pos = Vec2::new(x, y);

    for row in region.start_row..=region.end_row {
        for col in region.start_col..=region.end_col {
            for item in map.get_cell(row, col).iter() {
                let GameObjConfig::Tile(tile_config) = &(item.config.config) else {
                    continue;
                };

                if check_collide_obj(&pos, collide_span, &item.pos, tile_config.collide_span) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}
