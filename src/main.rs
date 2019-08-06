extern crate ggez;
extern crate itertools;
extern crate rand;

use std::env;
use std::path;

use ggez::{
    conf, event, graphics,
    graphics::{BlendMode, Color, DrawMode, DrawParam, Drawable, Font, Mesh, Rect, Text},
    mint::Point2,
    Context, ContextBuilder, GameResult,
};
use rand::{thread_rng, Rng};

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
enum Cell {
    Empty,
    Full(Color),
}

impl Cell {
    fn is_full(&self) -> bool {
        match self {
            &Cell::Full(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Shape {
    row: usize,
    column: usize,
    model: usize,
    orientation: usize,
}

impl Shape {
    fn generate() -> Shape {
        let mut rng = thread_rng();
        Shape {
            row: 0,
            column: 0,
            model: rng.gen_range(0, SHAPE_COUNT),
            orientation: 0,
        }
    }
}

impl Drawable for Shape {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let block_rect = Rect::new_i32(0, 0, BLOCK_SIZE - 2, BLOCK_SIZE - 2);
        let block_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, COLORS[self.model])?;
        let DrawParam { dest: offset, .. } = param;

        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                if SHAPES[self.model][self.orientation][i][j] == 1 {
                    let dest = Point2 {
                        x: offset.x + 1. + (BLOCK_SIZE * (self.column + i) as i32) as f32,
                        y: offset.y + 1. + (BLOCK_SIZE * (self.row + j) as i32) as f32,
                    };
                    let dp = DrawParam::new().dest(dest);
                    graphics::draw(ctx, &block_mesh, dp)?;
                }
            }
        }

        Ok(())
    }

    fn dimensions(&self, _: &mut Context) -> Option<Rect> {
        None
    }

    fn set_blend_mode(&mut self, _: Option<BlendMode>) {
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            cells: vec![vec![Cell::Empty; width]; height],
        }
    }

    fn clear_rows(&mut self) -> i32 {
        let mut rows_cleared = 0;
        let mut row = self.height as i32 - 1;
        while rows_cleared < row {
            let row_full = self.cells[row as usize].iter().all(Cell::is_full);
            if row_full {
                rows_cleared += 1;
            } else {
                // Only decrement the row if the current one wasn't cleared; without this,
                // sequential rows that were full wouldn't get cleared.
                row -= 1;
            }

            if rows_cleared > 0 && rows_cleared <= row {
                self.cells[row as usize] = self.cells[(row - rows_cleared) as usize].clone();
            }
        }

        for i in 0..=row {
            self.cells[i as usize] = vec![Cell::Empty; self.width];
        }

        rows_cleared
    }
}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let border_rect = Rect::new_i32(
            0, 0,
            self.width as i32 * BLOCK_SIZE,
            self.height as i32 * BLOCK_SIZE,
        );
        let border_mesh =
            Mesh::new_rectangle(ctx, DrawMode::stroke(1.), border_rect, graphics::WHITE)?;
        graphics::draw(ctx, &border_mesh, param)?;

        let block_rect = Rect::new_i32(0, 0, BLOCK_SIZE - 2, BLOCK_SIZE - 2);
        let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, graphics::WHITE)?;
        let DrawParam { dest: offset, .. } = param;
        for i in 0..self.height {
            for j in 0..self.width {
                match self.cells[i][j] {
                    Cell::Full(color) => {
                        let dest = Point2 {
                            x: offset.x + 1. + (BLOCK_SIZE * (j as i32)) as f32,
                            y: offset.y + 1. + (BLOCK_SIZE * (i as i32)) as f32,
                        };
                        let dp = DrawParam::new().dest(dest).color(color);
                        graphics::draw(ctx, &block_mesh, dp)?;
                    },
                    Cell::Empty => {},
                }
            }
        }

        Ok(())
    }

    fn dimensions(&self, _: &mut Context) -> Option<Rect> {
        Some(Rect::new_i32(0, 0, self.width as i32, self.height as i32))
    }

    fn set_blend_mode(&mut self, _: Option<BlendMode>) {
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

struct State {
    board: Board,
    shape: Shape,
}

impl State {
    fn new() -> State {
        State {
            board: Board::new(10, 20),
            shape: Shape::generate(),
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes", font, 12.));
        let board_param = DrawParam::new().dest(Point2 { x: 1., y: 100. });

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::draw(ctx, &self.board, board_param)?;
        graphics::draw(ctx, &self.shape, board_param)?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let state = &mut State::new();

    let c = conf::Conf::new();
    let mut cb = ContextBuilder::new("tetrominoes", "ajv").conf(c);

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we look in the cargo project for files.
    // Using a ContextBuilder is nice for this because it means that
    // it will look for a conf.toml or icon file or such in
    // this directory when the Context is created.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = cb.build()?;
    event::run(ctx, event_loop, state)
}