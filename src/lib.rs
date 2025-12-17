pub mod res {
    pub mod config;
    pub mod game_lib;
}

pub mod systems {
    pub mod setup;

    pub use setup::setup_game;
}

pub mod my_error;
pub mod utils;
