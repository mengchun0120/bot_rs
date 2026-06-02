use crate::config::{GameConfig, GenMapConfig};
use crate::systems::gen_map::generated_map::{BotConfigPair, GeneratedMap, TileConfigPair};
use rand::{Rng, rng, rngs::ThreadRng};

pub fn gen_island_map(
    game_config: &GameConfig,
    gen_map_config: &GenMapConfig,
    player_config: &BotConfigPair,
    ai_bot_configs: &Vec<BotConfigPair>,
    tile_configs: &Vec<TileConfigPair>,
) -> Option<GeneratedMap> {
    let mut r = rng();
    let mut y = 0.0;
    let end_x = gen_map_config.col_count as f32 * game_config.cell_size;
    let end_y = gen_map_config.row_count as f32 * game_config.cell_size;
    let mut map = GeneratedMap::new(gen_map_config.row_count, gen_map_config.col_count);

    Some(map)
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
