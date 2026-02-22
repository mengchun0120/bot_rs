pub mod game_plugin;
pub mod menu;
pub mod setup_app;
pub mod splash;

pub use game_plugin::*;
pub use menu::*;
pub use setup_app::*;
pub use splash::*;

mod game_play {
    pub mod add_new_objs;
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

    pub use add_new_objs::*;
    pub use cleanup::*;
    pub use gameover::*;
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
