pub mod config {
    pub mod game_config;
    pub mod game_map_config;
    pub mod game_obj_config;
    pub mod weapon_config;

    pub use game_config::*;
    pub use game_map_config::*;
    pub use game_obj_config::*;
    pub use weapon_config::*;
}

pub mod game {
    pub mod components;
    pub mod game_actions;
    pub mod game_obj;
    pub mod game_obj_side;
    pub mod game_obj_type;

    pub use components::*;
    pub use game_actions::*;
    pub use game_obj::*;
    pub use game_obj_side::*;
    pub use game_obj_type::*;
}

pub mod game_utils {
    pub mod despawn_pool;
    pub mod game_lib;
    pub mod game_map;
    pub mod game_obj_lib;
    pub mod map_pos;
    pub mod map_region;
    pub mod rect_region;

    pub use despawn_pool::*;
    pub use game_lib::*;
    pub use game_map::*;
    pub use game_obj_lib::*;
    pub use map_pos::*;
    pub use map_region::*;
    pub use rect_region::*;
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
    pub mod cleanup;
    pub mod process_cursor;
    pub mod process_key;
    pub mod process_mouse_button;
    pub mod setup;
    pub mod update_explosions;
    pub mod update_missiles;
    pub mod update_player;

    pub use cleanup::*;
    pub use process_cursor::*;
    pub use process_key::*;
    pub use process_mouse_button::*;
    pub use setup::*;
    pub use update_explosions::*;
    pub use update_missiles::*;
    pub use update_player::*;
}
