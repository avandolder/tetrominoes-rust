use ggez::graphics::Font;

pub struct World {
    pub font: Font,
    pub paused: bool,
    pub score: i32,
}
