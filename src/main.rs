mod my_error;
mod utils;

use crate::utils::*;
use bevy::{log::LogPlugin, prelude::*};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let _guard = setup_log(&args.log_path);

    info!("log has been setup");

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .run();

}
