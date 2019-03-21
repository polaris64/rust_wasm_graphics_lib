use wasm_bindgen::prelude::*;

use crate::types::ARGBColour;

#[wasm_bindgen]
pub struct Canvas {
    height: usize,
    width:  usize,

    buffer: Vec<u32>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            buffer: vec![0; width * height],
        }
    }

    pub fn buffer_ptr(&self) -> *const u32 {
        self.buffer.as_ptr()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn clear(&mut self, colour: &ARGBColour) {
        let colour: u32 = colour.into();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.buffer_index(x, y);
                self.buffer[idx] = colour;
            }
        }
    }

    pub fn buffer_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}

impl Canvas {
    pub fn buffer(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }
}
