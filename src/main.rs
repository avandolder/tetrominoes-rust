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
use rand::{seq::SliceRandom, thread_rng};

const BLOCK_SIZE: i32 = 16;
const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = 10;
const KEY_WAIT: f64 = 0.2;
const MOVE_WAIT: f64 = 0.75;
const FAST_MOVE_WAIT: f64 = 0.05;
const ORIENTATIONS: usize = 4;
const PIECES: usize = 7;
const PIECE_SIZE: usize = 4;

// PIECE contains all of the possible pieces in all of their
// possible orientations.
static PIECE: [[[[u8; PIECE_SIZE]; PIECE_SIZE]; ORIENTATIONS]; PIECES] = [
  [[[0, 0, 1, 0], // L piece
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
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 0, 0, 0], // J piece
    [1, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
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
    [0, 0, 0, 0]]],
  [[[0, 1, 0, 0], // T piece
    [1, 1, 1, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[0, 1, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0]],
   [[1, 1, 1, 0],
    [0, 1, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 1, 0, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0]]],
  [[[1, 1, 1, 1], // I piece
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
    [0, 0, 0, 0]],
   [[1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 0, 0, 0]]],
  [[[1, 1, 0, 0], // Z piece
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
  [[[0, 1, 1, 0], // S piece
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
  [[[1, 1, 0, 0], // O piece
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

static COLORS: [Color; PIECES] = [
    Color::new(1., 0.5, 0., 1.), // Orange (L)
    Color::new(0., 0., 1., 1.),  // Blue (J)
    Color::new(1., 0., 1., 1.),  // Purple (T)
    Color::new(0., 1., 1., 1.),  // Aqua (I)
    Color::new(1., 0., 0., 1.),  // Red (Z)
    Color::new(0., 1., 0., 1.),  // Green (S)
    Color::new(1., 1., 0., 1.),  // Yellow (O)
];

#[derive(Clone, Debug)]
enum Cell {
    Empty,
    Full(Color),
}

impl Cell {
    fn is_full(&self) -> bool {
        match *self {
            Cell::Full(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct Piece {
    row: i32,
    column: i32,
    shape: usize,
    orientation: usize,
    color: Color,
}

impl Piece {
    fn new(shape: usize) -> Piece {
        Piece {
            row: 0,
            column: 0,
            shape,
            orientation: 0,
            color: COLORS[shape],
        }
    }

    fn has_block(&self, row: usize, column: usize) -> bool {
        PIECE[self.shape][self.orientation][row][column] == 1
    }

    fn for_each_block<F>(&self, mut f: F)
    where
        F: FnMut(usize, usize),
    {
        for i in 0..PIECE_SIZE {
            for j in 0..PIECE_SIZE {
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

    fn rotate(&mut self, amt: i32) {
        let new_orientation = self.orientation as i32 + amt;
        if new_orientation < 0 {
            self.orientation = (ORIENTATIONS as i32 + new_orientation) as usize;
        } else {
            self.orientation = (new_orientation as usize) % ORIENTATIONS;
        }
    }

    fn prepare(mut self) -> Self {
        self.row = -2;
        self.column = (BOARD_WIDTH / 2 - PIECE_SIZE / 2) as i32;
        self
    }
}

impl Drawable for Piece {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let block_rect = Rect::new_i32(0, 0, BLOCK_SIZE - 2, BLOCK_SIZE - 2);
        let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, self.color)?;
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

    fn set_blend_mode(&mut self, _: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

fn generate_pieces() -> Vec<Piece> {
    let mut rng = thread_rng();
    let mut pieces: Vec<_> = (0..PIECES).map(Piece::new).collect();
    pieces.shuffle(&mut rng);
    pieces
}

fn make_ghost(p: &Piece) -> Piece {
    Piece {
        color: Color { a: 0.2, ..p.color },
        ..*p
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

    fn set_piece(&mut self, piece: &Piece) {
        let row = piece.row as usize;
        let column = piece.column as usize;
        piece.for_each_block(|i, j| {
            self.cells[row + i][column + j] = Cell::Full(piece.color);
        });
    }

    fn collides(&self, piece: &Piece) -> bool {
        let mut collides = false;
        piece.for_each_block(|i, j| {
            if piece.row + (i as i32) < 0 {
                return;
            }

            if piece.column + (j as i32) < 0
                || piece.column + (j as i32) >= self.width as i32
                || piece.row + (i as i32) >= self.height as i32
                || self.cells[(piece.row + (i as i32)) as usize]
                    [(piece.column + (j as i32)) as usize]
                    .is_full()
            {
                collides = true;
            }
        });
        collides
    }
}

impl Drawable for Board {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let border_rect = Rect::new_i32(
            0,
            0,
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
                    }
                    Cell::Empty => {}
                }
            }
        }

        Ok(())
    }

    fn dimensions(&self, _: &mut Context) -> Option<Rect> {
        Some(Rect::new_i32(0, 0, self.width as i32, self.height as i32))
    }

    fn set_blend_mode(&mut self, _: Option<BlendMode>) {}

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }
}

struct State {
    board: Board,
    piece: Piece,
    piece_bag: Vec<Piece>,
    ghost: Piece,
    move_dt: f64,
    key_dt: f64,
    score: i32,
}

impl State {
    fn new() -> State {
        let mut piece_bag = generate_pieces();
        let piece = piece_bag.pop().unwrap().prepare();
        let ghost = make_ghost(&piece);

        State {
            board: Board::new(BOARD_WIDTH, BOARD_HEIGHT),
            piece,
            piece_bag,
            ghost,
            move_dt: 0.,
            key_dt: KEY_WAIT,
            score: 0,
        }
    }

    fn rotate_piece(&mut self) {
        let mut new_piece = self.piece.clone();
        new_piece.rotate(1);
        for _ in 0..PIECE_SIZE {
            if !self.board.collides(&new_piece) {
                self.piece = new_piece;
                break;
            }
            new_piece = new_piece.shift(-1);
        }
    }

    fn drop_piece(&self, mut p: Piece) -> Piece {
        while !self.board.collides(&p) {
            p.row += 1;
        }
        p.row -= 1;
        p
    }

    fn update_ghost(&mut self) {
        self.ghost = self.drop_piece(make_ghost(&self.piece));
    }

    fn set_piece(&mut self) {
        if self.piece.row < 0 {
            println!("Game over!");
            std::process::exit(0);
        }

        self.board.set_piece(&self.piece);
        self.score += self.board.clear_rows().pow(2);
        self.piece = self.piece_bag.pop().unwrap().prepare();
        if self.piece_bag.is_empty() {
            self.piece_bag = generate_pieces();
        }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::duration_to_f64(timer::delta(ctx));
        self.move_dt += dt;
        if self.move_dt >= MOVE_WAIT
            || (keyboard::is_key_pressed(ctx, KeyCode::Down) && self.move_dt >= FAST_MOVE_WAIT)
        {
            self.piece.row += 1;

            if self.board.collides(&self.piece) {
                self.piece.row -= 1;
                self.set_piece();
            }

            self.move_dt = 0.;
        }

        self.key_dt += dt;
        if self.key_dt >= KEY_WAIT {
            if keyboard::is_key_pressed(ctx, KeyCode::Left)
                && !self.board.collides(&self.piece.clone().shift(-1))
            {
                self.piece.column -= 1;
                self.key_dt = 0.;
            } else if keyboard::is_key_pressed(ctx, KeyCode::Right)
                && !self.board.collides(&self.piece.clone().shift(1))
            {
                self.piece.column += 1;
                self.key_dt = 0.;
            }

            if keyboard::is_key_pressed(ctx, KeyCode::Up) {
                self.rotate_piece();
                self.key_dt = 0.;
            }

            if keyboard::is_key_pressed(ctx, KeyCode::Space) {
                self.piece = self.drop_piece(self.piece.clone());
                self.set_piece();
                self.key_dt = 0.;
            }
        }

        self.update_ghost();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let font = Font::new(ctx, "/FreeMono.ttf")?;
        let title = Text::new(("Tetrominoes", font, 12.));
        let next = Text::new(("Next Piece", font, 12.));
        let next_dp = DrawParam::new().dest(Point2 { x: 0., y: 100. });
        let score = Text::new((format!("Score: {}", self.score), font, 12.));
        let score_dp = DrawParam::new().dest(Point2 { x: 0., y: 50. });
        let next_piece_dp = DrawParam::new().dest(Point2 { x: 0., y: 120. });
        let board_dp = DrawParam::new().dest(Point2 { x: 100., y: 100. });

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &title, DrawParam::default())?;
        graphics::draw(ctx, &score, score_dp)?;
        graphics::draw(ctx, &next, next_dp)?;
        graphics::draw(ctx, self.piece_bag.last().unwrap(), next_piece_dp)?;
        graphics::draw(ctx, &self.board, board_dp)?;
        graphics::draw(ctx, &self.ghost, board_dp)?;
        graphics::draw(ctx, &self.piece, board_dp)?;
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
