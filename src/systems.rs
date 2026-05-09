pub mod game_plugin;
pub mod menu;
pub mod setup_app;
pub mod splash;

pub use game_plugin::game_plugin;
pub use menu::menu_plugin;
pub use setup_app::setup_app;
pub use splash::splash_plugin;

mod game_play {
    pub mod add_new_objs;
    pub mod check_game;
    pub mod cleanup;
    pub mod gameover;
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

    pub use add_new_objs::add_new_objs;
    pub use check_game::check_game;
    pub use cleanup::cleanup;
    pub use gameover::{gameover, wait_gameover};
    pub use process_cursor::process_cursor;
    pub use process_key::process_key;
    pub use process_mouse_button::process_mouse_button;
    pub use setup::setup_game;
    pub use update_ai::update_ai;
    pub use update_ai_bots::update_ai_bots;
    pub use update_missiles::update_missiles;
    pub use update_origin::update_origin;
    pub use update_player::update_player;
    pub use update_playout::update_playout;
}
