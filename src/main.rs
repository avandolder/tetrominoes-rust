mod board;
mod cell;
mod introscene;
mod mainscene;
mod pausescene;
mod piece;
mod world;

use std::env;
use std::path;

use ggez::{event, graphics::Font, Context, ContextBuilder, GameResult};
use ggez_goodies::scene::{Scene, SceneStack};

use introscene::IntroScene;
use world::World;

struct SceneManager {
    stack: SceneStack<World, ()>,
}

impl SceneManager {
    pub fn new(ctx: &mut Context, world: World, scene: Box<dyn Scene<World, ()>>) -> SceneManager {
        let mut stack = SceneStack::new(ctx, world);
        stack.push(scene);

        SceneManager { stack }
    }
}

impl event::EventHandler for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.stack.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.stack.draw(ctx);
        Ok(())
    }
}

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
    let world = World {
        font: Font::new(ctx, "/FreeMono.ttf")?,
        paused: false,
        score: 0,
    };
    let st = Box::new(IntroScene::new(ctx)?);
    let sm = &mut SceneManager::new(ctx, world, st);
    event::run(ctx, event_loop, sm)
}
