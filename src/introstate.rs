use ggez::{
    graphics::{self, DrawParam, Font, Text},
    input::keyboard::{self, KeyCode},
    Context, GameResult,
};
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::mainstate::MainState;
use crate::state::{State, Transition};
use crate::world::World;

pub struct IntroState;

impl IntroState {
    pub fn new(_ctx: &mut Context) -> GameResult<IntroState> {
        Ok(IntroState {})
    }
}

impl Scene<World, ()> for IntroState {
    fn update(&mut self, _world: &mut World, ctx: &mut Context) -> SceneSwitch<World, ()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            SceneSwitch::replace(MainState::new(ctx).unwrap())
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, _world: &mut World, ctx: &mut Context) -> GameResult {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes\nPress Enter to start", font, 12.));

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::present(ctx)
    }

    fn input(&mut self, _world: &mut World, _event: (), _started: bool) {}

    fn name(&self) -> &str {
        "Intro"
    }
}
