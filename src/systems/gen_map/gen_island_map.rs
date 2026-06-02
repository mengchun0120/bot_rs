use crate::config::{GameConfig, GenMapConfig, IslandGenMapAlgorithm};
use crate::systems::gen_map::generated_map::{
    BotConfigPair, GeneratedMap, GeneratedMapItem, TileConfigPair,
};
use rand::seq::SliceRandom;
use rand::{Rng, rng, rngs::ThreadRng};

pub fn gen_island_map(
    game_config: &GameConfig,
    gen_map_config: &GenMapConfig,
    player_config: &BotConfigPair,
    ai_bot_configs: &Vec<BotConfigPair>,
    tile_configs: &Vec<TileConfigPair>,
) -> Option<GeneratedMap> {
    let mut r = rng();

    let mut map = GeneratedMap::new(
        gen_map_config.row_count,
        gen_map_config.col_count,
        game_config.cell_size,
    );
    let mut y = 0.0;

    while true {
        let mut x = 0.0;


    }

    Some(map)
}

fn gen_island(
    map: &mut GeneratedMap,
    config: &IslandGenMapAlgorithm,
    x: f32,
    y: f32,
    tile_configs: &Vec<TileConfigPair>,
) -> Option<(f32, f32)> {
    let Some((dist_x, dist_y, tile_index, tile_count_x, tile_count_y)) =
        find_tile_for_island(map.width(), map.height(), config, x, y, tile_configs)
    else {
        return None;
    };

    if !add_tiles_to_island(
        map,
        x + dist_x,
        y + dist_y,
        tile_count_x,
        tile_count_y,
        &tile_configs[tile_index],
    ) {
        return None;
    }

    let span = tile_configs[tile_index].1.collide_span * 2.0;
    let new_x = x + dist_x + tile_count_x as f32 * span;
    let new_y = y + dist_y + tile_count_y as f32 * span;

    Some((new_x, new_y))
}

fn find_tile_for_island(
    width: f32,
    height: f32,
    config: &IslandGenMapAlgorithm,
    x: f32,
    y: f32,
    tile_configs: &Vec<TileConfigPair>,
) -> Option<(f32, f32, usize, usize, usize)> {
    let mut tile_indices: Vec<usize> = (0..tile_configs.len()).collect();
    let mut r = rng();

    tile_indices.shuffle(&mut r);

    for i in tile_indices {
        let tile_span = 2.0 * tile_configs[i].1.collide_span;
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

        return Some((dist_x, dist_y, i, tile_count_x, tile_count_y));
    }

    None
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
    tile_config: &TileConfigPair,
) -> bool {
    let span = tile_config.1.collide_span * 2.0;
    let mut y = y + tile_config.1.collide_span;

    for _ in 0..tile_count_y {
        let mut x1 = x + tile_config.1.collide_span;
        for _ in 0..tile_count_x {
            if !map.add(x1, y, GeneratedMapItem::Tile(tile_config.clone())) {
                return false;
            }
            x1 += span;
        }
        y += span;
    }

    true
}
