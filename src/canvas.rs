use wasm_bindgen::prelude::*;

use crate::types::ARGBColour;

#[wasm_bindgen]
/// A single buffer of 32-bit ARGB pixels with a fixed width and height
pub struct Canvas {
    height: usize,
    width:  usize,

    buffer: Vec<u32>,
}

#[wasm_bindgen]
impl Canvas {

    /// Create a new Canvas with a specified width and height in pixels
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            buffer: vec![0; width * height],
        }
    }

    /// Returns the index into the pixel buffer for a pixel given its x and y co-ordinates
    pub fn buffer_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    /// Returns a pointer to the Canvas pixel data
    pub fn buffer_ptr(&self) -> *const u32 {
        self.buffer.as_ptr()
    }

    /// Returns the Canvas's height in pixels
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the Canvas's width in pixels
    pub fn width(&self) -> usize {
        self.width
    }

    /// Clears the entire Canvas to a specified colour
    ///
    /// # Example:
    ///
    /// ```
    /// use rust_wasm_graphics_lib::canvas::Canvas;
    /// use rust_wasm_graphics_lib::types::ARGBColour;
    ///
    /// let mut c = Canvas::new(16, 16);
    /// c.clear(&ARGBColour::new(255, 255, 0, 0));
    /// ```
    pub fn clear(&mut self, colour: &ARGBColour) {
        let colour: u32 = colour.into();
        self.buffer.iter_mut().for_each(|x| *x = colour);
    }
}


impl Canvas {

    /// Returns a mutable reference to the pixel buffer for direct pixel access
    pub fn buffer(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }
}
