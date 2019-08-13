use ggez::{
    graphics,
    graphics::{BlendMode, Color, DrawMode, DrawParam, Drawable, Mesh, Rect},
    mint::Point2,
    Context, GameResult,
};
use rand::{seq::SliceRandom, thread_rng};

use crate::board::BOARD_WIDTH;
use crate::cell::CELL_SIZE;

const GHOST_ALPHA: f32 = 0.15;
const ORIENTATIONS: usize = 4;
const PIECES: usize = 7;
pub const PIECE_SIZE: usize = 4;

// PIECE contains all of the possible pieces in all of their
// possible orientations.
static PIECE: [[[[u8; PIECE_SIZE]; PIECE_SIZE]; ORIENTATIONS]; PIECES] = [
    // L piece
    [
        [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
    ],
    // J piece
    [
        [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
    ],
    // T piece
    [
        [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
    ],
    // I piece
    [
        [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
        [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
    ],
    // Z piece
    [
        [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]],
    ],
    // S piece
    [
        [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
        [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]],
    ],
    // O piece
    [
        [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    ],
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
pub struct Piece {
    pub row: i32,
    pub column: i32,
    pub shape: usize,
    pub orientation: usize,
    pub color: Color,
}

impl Piece {
    pub fn new(shape: usize) -> Piece {
        Piece {
            row: 0,
            column: 0,
            shape,
            orientation: 0,
            color: COLORS[shape],
        }
    }

    pub fn has_block(&self, row: usize, column: usize) -> bool {
        PIECE[self.shape][self.orientation][row][column] == 1
    }

    pub fn for_each_block<F>(&self, mut f: F)
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

    pub fn shift(self, amt: i32) -> Self {
        Piece {
            column: self.column + amt,
            ..self
        }
    }

    pub fn rotate(&mut self, amt: i32) {
        let new_orientation = self.orientation as i32 + amt;
        if new_orientation < 0 {
            self.orientation = (ORIENTATIONS as i32 + new_orientation) as usize;
        } else {
            self.orientation = (new_orientation as usize) % ORIENTATIONS;
        }
    }

    pub fn prepare(self) -> Self {
        Piece {
            row: -2,
            column: (BOARD_WIDTH / 2 - PIECE_SIZE / 2) as i32,
            ..self
        }
    }
}

impl Drawable for Piece {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let block_rect = Rect::new_i32(0, 0, CELL_SIZE - 2, CELL_SIZE - 2);
        let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, self.color)?;
        let DrawParam { dest: offset, .. } = param;

        self.for_each_block(|i, j| {
            let dest = Point2 {
                x: offset.x + 1. + (CELL_SIZE * (self.column + j as i32)) as f32,
                y: offset.y + 1. + (CELL_SIZE * (self.row + i as i32)) as f32,
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

pub fn generate_pieces() -> Vec<Piece> {
    let mut rng = thread_rng();
    let mut pieces: Vec<_> = (0..PIECES).map(Piece::new).collect();
    pieces.shuffle(&mut rng);
    pieces
}

pub fn make_ghost(p: &Piece) -> Piece {
    Piece {
        color: Color {
            a: GHOST_ALPHA,
            ..p.color
        },
        ..*p
    }
}
