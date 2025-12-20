pub mod config {
    pub mod game_config;
    pub mod game_map_config;
    pub mod game_obj_config;
}

pub mod game_utils {
    pub mod game_lib;
    pub mod game_map;
    pub mod game_obj_lib;
}

pub mod game {
    pub mod game_obj;
}

pub mod misc {
    pub mod my_error;
    pub mod utils;
}

pub mod systems {
    pub mod setup;

    pub use setup::setup_game;
}
