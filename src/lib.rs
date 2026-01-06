pub mod config {
    pub mod game_config;
    pub mod game_map_config;
    pub mod game_obj_config;
    pub mod weapon_config;
}

pub mod game {
    pub mod components;
    pub mod game_actions;
    pub mod game_obj;
}

pub mod game_utils {
    pub mod game_lib;
    pub mod game_map;
    pub mod game_obj_lib;
}

pub mod misc {
    pub mod collide;
    pub mod my_error;
    pub mod utils;
}

pub mod systems {
    pub mod process_key;
    pub mod process_mouse_button;
    pub mod setup;
    pub mod update_player;

    pub use process_key::process_key;
    pub use process_mouse_button::process_mouse_button;
    pub use setup::setup_game;
    pub use update_player::update_player;
}
