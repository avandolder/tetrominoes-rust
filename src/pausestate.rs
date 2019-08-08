use ggez::{
    graphics,
    graphics::{Color, DrawMode, DrawParam, Font, Mesh, Rect, Text},
    input::keyboard::{is_key_pressed, KeyCode},
    mint::Point2,
    Context, GameResult,
};

use crate::state::{State, StateRef, Transition};

pub struct PauseState {
    prev_state: Option<StateRef>,
}

impl PauseState {
    pub fn new() -> PauseState {
        PauseState { prev_state: None }
    }
}

impl State for PauseState {
    fn on_start(&mut self, _ctx: &mut Context, prev_state: Option<StateRef>) {
        self.prev_state = prev_state;
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<Transition> {
        if is_key_pressed(ctx, KeyCode::Return) {
            return Ok(Transition::Pop);
        }

        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Paused\nPress Enter to resume", font, 12.));
        let title_dp = DrawParam::new().dest(Point2 { x: 100., y: 50. });

        let Rect { w, h, .. } = graphics::screen_coordinates(ctx);
        let rect = Rect::new(0., 0., w, h);
        let color = Color::new(0., 0., 0., 0.75);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;

        self.prev_state.clone().unwrap().borrow_mut().draw(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        graphics::draw(ctx, &title, title_dp)?;
        Ok(())
    }
}
