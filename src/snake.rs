use smithay_client_toolkit::shm::slot::Buffer;

pub trait Game {
    /// Initialize the Game.
    fn start(&self);
    /// Draw the games state on a buffer.
    fn draw(&self, buffer: &mut Buffer);
    /// Move the game forward one tick.
    fn tick(&self);
}

pub struct Snake {
    length: u32,
    food_pos: (u32, u32),
    body: Vec<(u32, u32)>,
}

impl Game for Snake {
    fn start(&self) {}
    fn draw(&self, buffer: &mut Buffer) {
        buffer.height();
    }
    fn tick(&self) {}
}
