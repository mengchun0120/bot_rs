use crate::config::{
    GameConfig, GameObjConfig, GenMapAlgorithmConfig, GenMapConfig, IslandGenMapAlgorithm,
    NamedGameObjConfig,
};
use crate::misc::MyError;
use crate::systems::gen_map::gen_map_utils::gen_bots;
use crate::systems::gen_map::generated_map::GeneratedMap;
use bevy::prelude::*;
use rand::{Rng, rng, rngs::ThreadRng, seq::SliceRandom};

pub fn gen_island_map(
    game_config: &GameConfig,
    gen_map_config: &GenMapConfig,
    player_config: &NamedGameObjConfig,
    ai_bot_configs: &Vec<NamedGameObjConfig>,
    tile_configs: &Vec<NamedGameObjConfig>,
) -> Result<GeneratedMap, MyError> {
    let GenMapAlgorithmConfig::Island(config) = &gen_map_config.algorithm else {
        let msg = "gen_map_config doesn't have IslandGenMapAlgorithm".to_string();
        error!(msg);
        return Err(MyError::Other(msg));
    };
    let mut map = GeneratedMap::new(
        gen_map_config.row_count,
        gen_map_config.col_count,
        game_config.cell_size,
    );

    gen_islands(&mut map, config, tile_configs)?;
    gen_bots(
        &mut map,
        gen_map_config.ai_bot_count,
        player_config,
        ai_bot_configs,
    );

    Ok(map)
}

fn gen_islands(
    map: &mut GeneratedMap,
    config: &IslandGenMapAlgorithm,
    tile_configs: &Vec<NamedGameObjConfig>,
) -> Result<(), MyError> {
    let mut y = 0.0;

    loop {
        let mut x = 0.0;
        let mut next_y = y;

        let Some((new_x, new_y)) = gen_one_island(map, config, x, y, tile_configs)? else {
            break;
        };

        next_y = next_y.max(new_y);
        x = new_x;

        while let Some((new_x, new_y)) = gen_one_island(map, config, x, y, tile_configs)? {
            next_y = next_y.max(new_y);
            x = new_x;
        }

        y = next_y;
    }

    Ok(())
}

fn gen_one_island(
    map: &mut GeneratedMap,
    config: &IslandGenMapAlgorithm,
    x: f32,
    y: f32,
    tile_configs: &Vec<NamedGameObjConfig>,
) -> Result<Option<(f32, f32)>, MyError> {
    let Some((dist_x, dist_y, tile_index, tile_count_x, tile_count_y)) =
        find_tile_for_island(map.width(), map.height(), config, x, y, tile_configs)?
    else {
        return Ok(None);
    };

    let (new_x, new_y) = add_tiles_to_island(
        map,
        x + dist_x,
        y + dist_y,
        tile_count_x,
        tile_count_y,
        &tile_configs[tile_index],
    )?;

    Ok(Some((new_x, new_y)))
}

fn find_tile_for_island(
    width: f32,
    height: f32,
    config: &IslandGenMapAlgorithm,
    x: f32,
    y: f32,
    tile_configs: &Vec<NamedGameObjConfig>,
) -> Result<Option<(f32, f32, usize, usize, usize)>, MyError> {
    let mut tile_indices: Vec<usize> = (0..tile_configs.len()).collect();
    let mut r = rng();

    tile_indices.shuffle(&mut r);

    for i in tile_indices {
        let collide_span = tile_configs[i].tile_config()?.collide_span;
        let tile_span = 2.0 * collide_span;
        let Some((dist_x, tile_count_x)) = get_island_gap_span(
            &mut r,
            x,
            width,
            tile_span,
            config.min_island_dist,
            config.max_island_dist,
            config.min_island_width,
            config.max_island_width,
        ) else {
            continue;
        };
        let Some((dist_y, tile_count_y)) = get_island_gap_span(
            &mut r,
            y,
            height,
            tile_span,
            config.min_island_dist,
            config.max_island_dist,
            config.min_island_height,
            config.max_island_height,
        ) else {
            continue;
        };

        return Ok(Some((dist_x, dist_y, i, tile_count_x, tile_count_y)));
    }

    Ok(None)
}

fn get_island_gap_span(
    r: &mut ThreadRng,
    start: f32,
    end: f32,
    tile_span: f32,
    min_island_dist: f32,
    max_island_dist: f32,
    min_island_span: f32,
    max_island_span: f32,
) -> Option<(f32, usize)> {
    let min_island_tile_count = (min_island_span / tile_span).ceil().max(1.0) as usize;
    let max_island_tile_count = (max_island_span / tile_span).floor() as usize;

    if min_island_tile_count > max_island_tile_count {
        return None;
    }

    let min_island_span = min_island_tile_count as f32 * tile_span;
    let max_island_span = max_island_tile_count as f32 * tile_span;

    if end - start - min_island_span < 2.0 * min_island_dist {
        return None;
    }

    let max_island_dist = (end - start - min_island_span - min_island_dist).min(max_island_dist);
    let island_dist = r.random_range(min_island_dist..max_island_dist);
    let max_island_span = (end - start - island_dist - min_island_dist).min(max_island_span);
    let max_island_tile_count = (max_island_span / tile_span).floor() as usize;
    let island_tile_count = r.random_range(min_island_tile_count..=max_island_tile_count);

    Some((island_dist, island_tile_count))
}

fn add_tiles_to_island(
    map: &mut GeneratedMap,
    x: f32,
    y: f32,
    tile_count_x: usize,
    tile_count_y: usize,
    config: &NamedGameObjConfig,
) -> Result<(f32, f32), MyError> {
    let collide_span = config.tile_config()?.collide_span;
    let span = collide_span * 2.0;
    let mut y1 = y + collide_span;

    for _ in 0..tile_count_y {
        let mut x1 = x + collide_span;
        for _ in 0..tile_count_x {
            map.add(x1, y1, config.clone())?;
            x1 += span;
        }
        y1 += span;
    }

    let new_x = x + tile_count_x as f32 * span;
    let new_y = y + tile_count_y as f32 * span;

    Ok((new_x, new_y))
}
