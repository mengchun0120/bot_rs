use crate::config::GameConfig;
use crate::game_utils::GameLib;
use crate::misc::{states::AppState, utils::Args};
use bevy::prelude::*;
use std::path::PathBuf;

pub fn setup_app(
    args: Res<Args>,
    mut window: Single<&mut Window>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_app: MessageWriter<AppExit>,
) {
    let Some(game_lib) = load_game_lib(
        &args.config_path,
        asset_server.as_ref(),
        layouts.as_mut(),
        &mut exit_app,
    ) else {
        return;
    };

    init_window(&game_lib.game_config, window.as_mut());
    commands.spawn(Camera2d);
    commands.insert_resource(game_lib);
    app_state.set(AppState::Splash);

    info!("App setup finished");
}

fn load_game_lib(
    config_path: &PathBuf,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    exit_app: &mut MessageWriter<AppExit>,
) -> Option<GameLib> {
    match GameLib::load(config_path, asset_server, layouts) {
        Ok(lib) => Some(lib),
        Err(err) => {
            error!("Failed to load GameLib: {}", err);
            exit_app.write(AppExit::error());
            None
        }
    }
}

fn init_window(game_config: &GameConfig, window: &mut Window) {
    window
        .resolution
        .set(game_config.window_width(), game_config.window_height());
}
