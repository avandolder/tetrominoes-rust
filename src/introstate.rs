use ggez::{
    graphics::{self, DrawParam, Font, Text},
    input::keyboard::{self, KeyCode},
    Context, GameResult,
};

use crate::mainstate::MainState;
use crate::state::{State, Transition};

pub struct IntroState;

impl IntroState {
    pub fn new(_ctx: &mut Context) -> GameResult<IntroState> {
        Ok(IntroState {})
    }
}

impl State for IntroState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition> {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            Transition::switch(MainState::new(ctx)?)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes\nPress Enter to start", font, 12.));

        graphics::draw(ctx, &title, DrawParam::default())?;
        Ok(())
    }
}
