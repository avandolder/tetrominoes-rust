mod board;
mod cell;
mod introstate;
mod mainstate;
mod pausestate;
mod piece;
mod state;

use std::env;
use std::path;

use ggez::{event, ContextBuilder, GameResult};

use introstate::IntroState;
use state::StateManager;

fn main() -> GameResult {
    let resource_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        path::PathBuf::from(manifest_dir).join("resources")
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("tetrominoes", "ajv")
        .add_resource_path(resource_path)
        .with_conf_file(true);

    let (ctx, event_loop) = &mut cb.build()?;
    let st = Box::new(IntroState::new(ctx)?);
    let sm = &mut StateManager::new(ctx, st);
    event::run(ctx, event_loop, sm)
}
