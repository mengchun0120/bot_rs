pub mod res {
    pub mod game_config;
    pub mod game_lib;
    pub mod game_obj_config;
}

pub mod systems {
    pub mod setup;

    pub use setup::setup_game;
}

pub mod my_error;
pub mod utils;
