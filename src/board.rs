use ggez::{
    graphics,
    graphics::{BlendMode, DrawMode, DrawParam, Drawable, Mesh, Rect},
    mint::Point2,
    Context, GameResult,
};

use crate::cell::{Cell, CELL_SIZE};
use crate::piece::Piece;

pub const BOARD_HEIGHT: usize = 20;
pub const BOARD_WIDTH: usize = 10;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    block_mesh: Mesh,
    border_mesh: Mesh,
}

impl Board {
    pub fn new(ctx: &mut Context, width: usize, height: usize) -> GameResult<Board> {
        let block_rect = Rect::new_i32(0, 0, CELL_SIZE - 2, CELL_SIZE - 2);
        let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), block_rect, graphics::WHITE)?;

        let border_rect = Rect::new_i32(0, 0, width as i32 * CELL_SIZE, height as i32 * CELL_SIZE);
        let border_mesh =
            Mesh::new_rectangle(ctx, DrawMode::stroke(1.), border_rect, graphics::WHITE)?;

        Ok(Board {
            width,
            height,
            cells: vec![vec![Cell::Empty; width]; height],
            block_mesh,
            border_mesh,
        })
    }

    pub fn clear_rows(&mut self) -> i32 {
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

    pub fn set_piece(&mut self, piece: &Piece) {
        let row = piece.row as usize;
        let column = piece.column as usize;
        piece.for_each_block(|i, j| {
            self.cells[row + i][column + j] = Cell::Full(piece.color);
        });
    }

    pub fn collides(&self, piece: &Piece) -> bool {
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
        graphics::draw(ctx, &self.border_mesh, param)?;

        let DrawParam { dest: offset, .. } = param;
        for i in 0..self.height {
            for j in 0..self.width {
                match self.cells[i][j] {
                    Cell::Full(color) => {
                        let dest = Point2 {
                            x: offset.x + 1. + (CELL_SIZE * (j as i32)) as f32,
                            y: offset.y + 1. + (CELL_SIZE * (i as i32)) as f32,
                        };
                        let dp = DrawParam::new().dest(dest).color(color);
                        graphics::draw(ctx, &self.block_mesh, dp)?;
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
