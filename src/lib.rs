pub mod config {
    pub mod game_config;
    pub mod game_map_config;
    pub mod game_obj_config;
    pub mod gun_config;
}

pub mod game_utils {
    pub mod game_lib;
    pub mod game_map;
    pub mod game_obj_lib;
}

pub mod game {
    pub mod components;
    pub mod game_obj;
}

pub mod misc {
    pub mod my_error;
    pub mod utils;
}

pub mod systems {
    pub mod process_input;
    pub mod setup;

    pub use process_input::process_input;
    pub use setup::setup_game;
}
