use ggez::{
    graphics::{self, DrawParam, Text},
    input::keyboard::{self, KeyCode},
    Context, GameResult,
};
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::mainscene::MainScene;
use crate::world::World;

pub struct IntroScene;

impl IntroScene {
    pub fn new(_ctx: &mut Context) -> GameResult<IntroScene> {
        Ok(IntroScene {})
    }
}

impl Scene<World, ()> for IntroScene {
    fn update(&mut self, _world: &mut World, ctx: &mut Context) -> SceneSwitch<World, ()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            SceneSwitch::replace(MainScene::new(ctx).unwrap())
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, world: &mut World, ctx: &mut Context) -> GameResult {
        let title = Text::new(("Tetrominoes\nPress Enter to start", world.font, 12.));

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::present(ctx)
    }

    fn input(&mut self, _world: &mut World, _event: (), _started: bool) {}

    fn name(&self) -> &str {
        "Intro"
    }
}
