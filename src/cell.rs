use ggez::graphics::Color;

pub const CELL_SIZE: i32 = 16;

#[derive(Clone, Debug)]
pub enum Cell {
    Empty,
    Full(Color),
}

impl Cell {
    pub fn is_full(&self) -> bool {
        match *self {
            Cell::Full(_) => true,
            _ => false,
        }
    }
}
