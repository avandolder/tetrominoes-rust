extern crate ggez;
extern crate itertools;
extern crate rand;

use std::env;
use std::path;

use ggez::{
    conf, event, graphics,
    graphics::{BlendMode, Color, DrawMode, DrawParam, Drawable, Font, Mesh, Rect, Text},
    input::{keyboard, keyboard::KeyCode},
    mint::Point2,
    timer, Context, ContextBuilder, GameResult,
};
use rand::{thread_rng, Rng};

const BLOCK_SIZE: i32 = 16;
const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = 10;
const KEY_WAIT: f64 = 0.2;
const MOVE_WAIT: f64 = 0.75;
const MOVE_WAIT_FAST: f64 = 0.05;
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
    Color::new(1., 0.5, 0., 1.), // Orange (L)
    Color::new(0., 0., 1., 1.), // Blue (J)
    Color::new(1., 0., 1., 1.), // Purple (T)
    Color::new(0., 1., 1., 1.), // Aqua (I)
    Color::new(1., 0., 0., 1.), // Red (Z)
    Color::new(0., 1., 0., 1.), // Green (S)
    Color::new(1., 1., 0., 1.), // Yellow (O)
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

#[derive(Clone, Debug)]
struct Shape {
    row: i32,
    column: i32,
    model: usize,
    orientation: usize,
}

impl Shape {
    fn generate() -> Shape {
        let mut rng = thread_rng();
        Shape {
            row: -2,
            column: 0,
            model: rng.gen_range(0, SHAPE_COUNT),
            orientation: 0,
        }
    }

    fn has_block(&self, row: usize, column: usize) -> bool {
        SHAPES[self.model][self.orientation][row][column] == 1
    }

    fn color(&self) -> Color {
        COLORS[self.model]
    }

    fn for_each_block<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        for i in 0..SHAPE_SIZE {
            for j in 0..SHAPE_SIZE {
                if self.has_block(i, j) {
                    f(i, j);
                }
            }
        }
    }

    fn shift(mut self, amt: i32) -> Self {
        self.column += amt;
        self
    }
}

impl Drawable for Shape {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let block_rect = Rect::new_i32(0, 0, BLOCK_SIZE - 2, BLOCK_SIZE - 2);
        let block_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, COLORS[self.model])?;
        let DrawParam { dest: offset, .. } = param;

        self.for_each_block(|i, j| {
            let dest = Point2 {
                x: offset.x + 1. + (BLOCK_SIZE * (self.column + j as i32)) as f32,
                y: offset.y + 1. + (BLOCK_SIZE * (self.row + i as i32)) as f32,
            };
            let dp = DrawParam::new().dest(dest);
            graphics::draw(ctx, &block_mesh, dp).unwrap();
        });

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

    fn set_shape(&mut self, shape: &Shape) {
        let row = shape.row as usize;
        let column = shape.column as usize;
        shape.for_each_block(|i, j| {
            self.cells[row + i][column + j] = Cell::Full(shape.color());
        });
    }

    fn collides(&self, shape: &Shape) -> bool {
        let mut collides = false;
        shape.for_each_block(|i, j| {
            if shape.row + (i as i32) < 0 {
                return;
            }

            if shape.column + (j as i32) < 0
                || shape.column + (j as i32) >= self.width as i32
                || shape.row + (i as i32) >= self.height as i32
                || self.cells[(shape.row + (i as i32)) as usize]
                    [(shape.column + (j as i32)) as usize]
                    .is_full() {
                collides = true;
            }
        });
        collides
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
    next_shape: Shape,
    move_dt: f64,
    key_dt: f64,
}

impl State {
    fn new() -> State {
        State {
            board: Board::new(BOARD_WIDTH, BOARD_HEIGHT),
            shape: Shape::generate(),
            next_shape: Shape::generate(),
            move_dt: 0.,
            key_dt: KEY_WAIT,
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::duration_to_f64(timer::delta(ctx));
        self.move_dt += dt;
        if self.move_dt >= MOVE_WAIT || (
                keyboard::is_key_pressed(ctx, KeyCode::Down) &&
                self.move_dt >= MOVE_WAIT_FAST) {
            self.shape.row += 1;

            if self.board.collides(&self.shape) {
                self.shape.row -= 1;
                if self.shape.row < 0 {
                    println!("Game over!");
                    std::process::exit(0);
                }

                self.board.set_shape(&self.shape);
                self.board.clear_rows();
                self.shape = self.next_shape.clone();
                self.next_shape = Shape::generate();
            }

            self.move_dt = 0.;
        }

        self.key_dt += dt;
        if self.key_dt >= KEY_WAIT {
            if keyboard::is_key_pressed(ctx, KeyCode::Left) &&
                    !self.board.collides(&self.shape.clone().shift(-1)) {
                self.shape.column -= 1;
                self.key_dt = 0.;
            } else if keyboard::is_key_pressed(ctx, KeyCode::Right) &&
                    !self.board.collides(&self.shape.clone().shift(1)) {
                self.shape.column += 1;
                self.key_dt = 0.;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes", font, 12.));
        let next_msg = Text::new(("Next Shape", font, 12.));
        let next_msg_dp = DrawParam::new().dest(Point2 { x: 0., y: 100. });
        let next_shape_dp = DrawParam::new().dest(Point2 { x: 0., y: 160. });
        let board_dp = DrawParam::new().dest(Point2 { x: 100., y: 100. });

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::draw(ctx, &next_msg, next_msg_dp)?;
        graphics::draw(ctx, &self.next_shape, next_shape_dp)?;
        graphics::draw(ctx, &self.board, board_dp)?;
        graphics::draw(ctx, &self.shape, board_dp)?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let state = &mut State::new();

    let c = conf::Conf::new();
    let mut cb = ContextBuilder::new("tetrominoes", "ajv").conf(c);

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let path = path::PathBuf::from(manifest_dir).join("resources");
        cb = cb.add_resource_path(path);
    }

    let (ref mut ctx, ref mut event_loop) = cb.build()?;
    event::run(ctx, event_loop, state)
}