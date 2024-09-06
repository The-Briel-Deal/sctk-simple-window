use std::usize;

use wayland_client::protocol::wl_shm::Format;

use crate::helper::position::Position;

pub trait Game {
    /// Initialize the Game.
    fn start(&self);
    /// Draw the games state on a buffer.
    fn draw(&self, width: u32, height: u32, format: Format, canvas: &mut [u8]);
    /// Move the game forward one tick.
    fn tick(&self);
}

pub struct Snake {
    length: u32,
    food_pos: (u32, u32),
    body: Vec<(u32, u32)>,
    board_size: (u32, u32),
}
impl Game for Snake {
    fn start(&self) {}
    fn draw(&self, width: u32, height: u32, format: Format, canvas: &mut [u8]) {
        assert_eq!(format, Format::Argb8888); // Currently only supports Argb8888
        let center = Position {
            x: width / 2,
            y: height / 2,
        };
        canvas
            .chunks_exact_mut(4)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                let pos = Position {
                    x: (chunk_index % width as usize) as u32,
                    y: (chunk_index / width as usize) as u32,
                };
                chunk.iter_mut().enumerate().for_each(|(byte_index, byte)| {
                    if byte_index == 3 { // Always set alpha byte to 255
                        *byte = 0xFF;
                        return;
                    }
                    if pos.near(&center, 30) {
                        *byte = 0xFF;
                    } else {
                        *byte = 0x00;
                    }
                })
            })
    }
    fn tick(&self) {}
}

impl Snake {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            body: vec![],
            food_pos: (0, 0),
            length: 0,
            board_size: (20, 20),
        }
    }
}
