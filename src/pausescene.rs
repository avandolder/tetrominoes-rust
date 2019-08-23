use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    input::keyboard::{self, KeyCode},
    mint::Point2,
    Context, GameResult,
};
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::world::World;

pub struct PauseScene;

impl PauseScene {
    pub fn new(_ctx: &mut Context) -> GameResult<PauseScene> {
        Ok(PauseScene {})
    }
}

impl Scene<World, ()> for PauseScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> SceneSwitch<World, ()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            world.paused = false;
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, world: &mut World, ctx: &mut Context) -> GameResult {
        let msg = Text::new(("Paused\nPress Enter to resume", world.font, 12.));
        let msg_dp = DrawParam::new().dest(Point2 { x: 100., y: 50. });

        let Rect { w, h, .. } = graphics::screen_coordinates(ctx);
        let rect = Rect::new(0., 0., w, h);
        let color = Color::new(0., 0., 0., 0.75);
        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;

        graphics::draw(ctx, &mesh, DrawParam::default())?;
        graphics::draw(ctx, &msg, msg_dp)?;
        graphics::present(ctx)
    }

    fn input(&mut self, _world: &mut World, _event: (), _started: bool) {}

    fn name(&self) -> &str {
        "Pause"
    }

    fn draw_previous(&self) -> bool {
        true
    }
}
