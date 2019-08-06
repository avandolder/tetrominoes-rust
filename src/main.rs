extern crate ggez;

use ggez::{
    conf, event, graphics,
    graphics::{Color, DrawParam, Font, Text},
    Context, ContextBuilder, GameResult,
};

const BLOCK_SIZE: i32 = 16;
const ORIENTATIONS: usize = 4;
const SHAPE_COUNT: usize = 7;
const SHAPE_SIZE: usize = 4;

// SHAPES contains all of the possible shapes in all of their
// possible orientations.
static SHAPES: [[[[u8; SHAPE_SIZE]; SHAPE_SIZE]; ORIENTATIONS]; SHAPE_COUNT] = [
  [[[1, 0, 0, 0], // L shape
    [1, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0]],
   [[0, 0, 1, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 1, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]]],
  [[[0, 1, 0, 0], // J shape
    [0, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 1, 1, 0], // T shape
    [0, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 0, 0, 0], // I shape
    [1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0]],
   [[1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0]],
   [[1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 1, 0, 0], // Z shape
    [0, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
    [1, 1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
    [1, 1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0]]],
  [[[0, 1, 1, 0], // S shape
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 1, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 1, 0, 0], // O shape
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]]],
];

static COLORS: [Color; 7] = [
    Color::new(1., 0.5, 0., 1.), // Orange
    Color::new(0., 0., 1., 1.), // Blue
    Color::new(1., 0., 1., 1.), // Purple
    Color::new(0., 1., 1., 1.), // Aqua
    Color::new(1., 0., 0., 1.), // Red
    Color::new(0., 1., 0., 1.), // Green
    Color::new(1., 1., 0., 1.), // Yellow
];

#[derive(Clone, Debug)]
enum CellState {
    Empty,
    Full,
}

#[derive(Clone, Debug)]
struct Cell {
    row: usize,
    column: usize,
    state: CellState,
}

#[derive(Clone, Debug)]
struct Shape {
    row: usize,
    column: usize,
    model: usize,
    orientation: usize,
}

struct State {}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes", font, 12.));

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::present(ctx)
    }
}

pub fn main() {
    let state = &mut State {};

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("tetrominoes", "ajv")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}