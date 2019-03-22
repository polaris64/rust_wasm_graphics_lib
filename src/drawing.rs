use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;
use crate::types::ARGBColour;

#[wasm_bindgen]
/// Draws a filled rectangle of a given colour to a Canvas
///
/// # Arguments:
///
///   - `c`: target Canvas
///   - `col`: colour to use for fill
///   - `x1`: x co-ordinate of top-left corner
///   - `y1`: y co-ordinate of top-left corner
///   - `x2`: x co-ordinate of bottom-right corner
///   - `y2`: y co-ordinate of bottom-right corner
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::fill_rect;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(16, 16);
/// fill_rect(&mut c, &ARGBColour::new(255, 255, 0, 0), 4, 4, 12, 12);
/// ```
pub fn fill_rect(c: &mut Canvas, col: &ARGBColour, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize) {
    if x1 > x2 {
        let temp = x1;
        x1 = x2;
        x2 = temp;
    }
    if y1 > y2 {
        let temp = y1;
        y1 = y2;
        y2 = temp;
    }
    if x1 > c.width() { x1 = c.width() }
    if y1 > c.height() { y1 = c.height() }
    if x2 > c.width() { x2 = c.width() }
    if y2 > c.height() { y2 = c.height() }

    let col: u32 = col.into();
    for y in y1..y2 {
        for x in x1..x2 {
            let idx = c.buffer_index(x, y);
            c.buffer()[idx] = col;
        }
    }
}

#[wasm_bindgen]
/// Draws a horizontal line of a given colour to a Canvas
///
/// # Arguments:
///
///   - `c`: target Canvas
///   - `col`: colour to use for line
///   - `x1`: x co-ordinate of line start
///   - `y`: y co-ordinate
///   - `x2`: x co-ordinate of line end
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::h_line;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(16, 16);
/// h_line(&mut c, &ARGBColour::new(255, 255, 0, 0), 4, 8, 12);
/// ```
pub fn h_line(c: &mut Canvas, col: &ARGBColour, mut x1: usize, y: usize, mut x2: usize) {
    if y >= c.height() {
        return;
    }
    if x1 > x2 {
        let temp = x1;
        x1 = x2;
        x2 = temp;
    }
    if x1 > c.width() {
        return;
    }
    if x2 >= c.width() { x2 = c.width() - 1 }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x1, y);
    for _ in x1..x2 + 1 {
        c.buffer()[idx] = col;
        idx += 1;
    }
}

#[wasm_bindgen]
/// Draws a vertical line of a given colour to a Canvas
///
/// # Arguments:
///
///   - `c`: target Canvas
///   - `col`: colour to use for line
///   - `x`: x co-ordinate
///   - `y1`: y co-ordinate of line start
///   - `x2`: y co-ordinate of line end
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::v_line;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(16, 16);
/// v_line(&mut c, &ARGBColour::new(255, 255, 0, 0), 8, 4, 12);
/// ```
pub fn v_line(c: &mut Canvas, col: &ARGBColour, x: usize, mut y1: usize, mut y2: usize) {
    if x >= c.width() {
        return;
    }
    if y1 > y2 {
        let temp = y1;
        y1 = y2;
        y2 = temp;
    }
    if y1 > c.height() {
        return;
    }
    if y2 >= c.height() { y2 = c.height() - 1 }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x, y1);
    for _ in y1..y2 + 1 {
        c.buffer()[idx] = col;
        idx += c.width()
    }
}
