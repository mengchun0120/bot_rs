pub mod ai {
    pub mod ai_action;
    pub mod ai_engine;
    pub mod chase_shoot_ai_engine;

    pub use ai_action::*;
    pub use ai_engine::*;
    pub use chase_shoot_ai_engine::*;
}

pub mod config {
    pub mod ai_config;
    pub mod game_config;
    pub mod game_map_config;
    pub mod game_obj_config;
    pub mod weapon_config;

    pub use ai_config::*;
    pub use game_config::*;
    pub use game_map_config::*;
    pub use game_obj_config::*;
    pub use weapon_config::*;
}

pub mod game {
    pub mod components;
    pub mod create_obj;
    pub mod game_actions;
    pub mod game_obj;
    pub mod game_obj_side;
    pub mod game_obj_type;
    pub mod move_obj;
    pub mod on_death_action;
    pub mod playout;
    pub mod shoot;

    pub use components::*;
    pub use create_obj::*;
    pub use game_actions::*;
    pub use game_obj::*;
    pub use game_obj_side::*;
    pub use game_obj_type::*;
    pub use move_obj::*;
    pub use on_death_action::*;
    pub use playout::*;
    pub use shoot::*;
}

pub mod game_utils {
    pub mod despawn_pool;
    pub mod game_lib;
    pub mod game_map;
    pub mod game_obj_lib;
    pub mod map_pos;
    pub mod map_region;
    pub mod new_obj_queue;
    pub mod rect_region;
    pub mod world_info;

    pub use despawn_pool::*;
    pub use game_lib::*;
    pub use game_map::*;
    pub use game_obj_lib::*;
    pub use map_pos::*;
    pub use map_region::*;
    pub use new_obj_queue::*;
    pub use rect_region::*;
    pub use world_info::*;
}

pub mod misc {
    pub mod collide;
    pub mod my_error;
    pub mod utils;

    pub use collide::*;
    pub use my_error::*;
    pub use utils::*;
}

pub mod systems {
    pub mod add_new_objs;
    pub mod cleanup;
    pub mod process_cursor;
    pub mod process_key;
    pub mod process_mouse_button;
    pub mod setup;
    pub mod update_ai;
    pub mod update_ai_bots;
    pub mod update_missiles;
    pub mod update_origin;
    pub mod update_player;
    pub mod update_playout;

    pub use add_new_objs::*;
    pub use cleanup::*;
    pub use process_cursor::*;
    pub use process_key::*;
    pub use process_mouse_button::*;
    pub use setup::*;
    pub use update_ai::*;
    pub use update_ai_bots::*;
    pub use update_missiles::*;
    pub use update_origin::*;
    pub use update_player::*;
    pub use update_playout::*;
}
