//! Drawing functions used to draw primites to a [`Canvas`]
//!
//! [`Canvas`]: ../canvas/struct.Canvas.html

use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;
use crate::types::ARGBColour;

#[wasm_bindgen]
/// Draws a filled rectangle of a given colour to a [`Canvas`]
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
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
        std::mem::swap(&mut x1, &mut x2);
    }
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }
    if x1 >= c.width() { x1 = c.width() - 1 }
    if y1 >= c.height() { y1 = c.height() - 1 }
    if x2 >= c.width() { x2 = c.width() - 1 }
    if y2 >= c.height() { y2 = c.height() - 1 }

    let col: u32 = col.into();
    let w = c.width();

    // Split buffer into chunks of "width", skip until the top of the rectangle and iterate over
    // all rows (y2 - y1) to get each scanline.  Then, set pixels in the scanline in the interval
    // [x1,x2].
    c.buffer()
        .as_mut_slice()
        .chunks_mut(w)
        .skip(y1)
        .take(y2 - y1 + 1)
        .for_each(|scanline|
            scanline.iter_mut()
                .skip(x1)
                .take(x2 - x1 + 1)
                .for_each(|x| *x = col)
        );
}

#[wasm_bindgen]
/// Draws a horizontal line of a given colour to a [`Canvas`]
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
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
        std::mem::swap(&mut x1, &mut x2);
    }
    if x1 > c.width() {
        return;
    }
    if x2 >= c.width() { x2 = c.width() - 1 }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x1, y);
    for _ in x1..=x2 {
        c.buffer()[idx] = col;
        idx += 1;
    }
}

#[wasm_bindgen]
/// Draws a vertical line of a given colour to a [`Canvas`]
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
///   - `col`: colour to use for line
///   - `x`: x co-ordinate
///   - `y1`: y co-ordinate of line start
///   - `y2`: y co-ordinate of line end
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
        std::mem::swap(&mut y1, &mut y2);
    }
    if y1 > c.height() {
        return;
    }
    if y2 >= c.height() { y2 = c.height() - 1 }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x, y1);
    for _ in y1..=y2 {
        c.buffer()[idx] = col;
        idx += c.width()
    }
}

fn plot_line_low(c: &mut Canvas, col: u32, x1: usize, y1: usize, x2: usize, y2: usize) {
    let dx: isize = (x2 - x1) as isize;
    let mut dy: isize = (y2 - y1) as isize;
    let mut yi: isize = 1isize;
    if dy < 0 {
        yi = -1isize;
        dy = -dy;
    }
    let mut d: isize = 2isize * (dy as isize) - (dx as isize);
    let mut y: isize = y1 as isize;
    for x in x1..=x2 {
        if y < 0 { break; }
        if y >= c.height() as isize { break; }
        let idx = c.buffer_index(x, y as usize);
        c.buffer()[idx] = col;
        if d > 0 {
            y += yi;
            d -= 2 * dx;
        }
        d += 2 * dy;
    }
}

fn plot_line_high(c: &mut Canvas, col: u32, x1: usize, y1: usize, x2: usize, y2: usize) {
    let mut dx: isize = (x2 - x1) as isize;
    let dy: isize = (y2 - y1) as isize;
    let mut xi: isize = 1isize;
    if dx < 0 {
        xi = -1isize;
        dx = -dx;
    }
    let mut d: isize = 2isize * (dx as isize) - (dy as isize);
    let mut x: isize = x1 as isize;
    for y in y1..=y2 {
        if x < 0 { break; }
        if x >= c.width() as isize { break; }
        let idx = c.buffer_index(x as usize, y);
        c.buffer()[idx] = col;
        if d > 0 {
            x += xi;
            d -= 2 * dy;
        }
        d += 2 * dx;
    }
}

/// Draws a line of a given colour to a [`Canvas`] using Bresenham's line drawing algorithm.
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
///   - `col`: colour to use for line
///   - `x1`: x co-ordinate of line start
///   - `y1`: y co-ordinate of line start
///   - `x2`: x co-ordinate of line end
///   - `y2`: y co-ordinate of line end
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::line_bresenham;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(128, 128);
/// line_bresenham(&mut c, &ARGBColour::new(255, 255, 0, 0), 10, 20, 110, 120);
/// ```
pub fn line_bresenham(c: &mut Canvas, col: &ARGBColour, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize) {
    if x2 > x1 && x1 > c.width() { return; }
    if y2 > y1 && y1 > c.height() { return; }
    if x1 > x2 && x2 > c.width() { return; }
    if y1 > y2 && y2 > c.height() { return; }
    if x1 > x2 && x1 >= c.width() { x1 = c.width() - 1 }
    if y1 > y2 && y1 >= c.height() { y1 = c.height() - 1 }
    if x2 > x1 && x2 >= c.width() { x2 = c.width() - 1 }
    if y2 > y1 && y2 >= c.height() { y2 = c.height() - 1 }

    let col: u32 = col.into();

    if ((y2 as isize) - (y1 as isize)).abs() < ((x2 as isize) - (x1 as isize)).abs() {
        if x1 > x2 {
            plot_line_low(c, col, x2, y2, x1, y1);
        } else {
            plot_line_low(c, col, x1, y1, x2, y2);
        }
    } else if y1 > y2 {
        plot_line_high(c, col, x2, y2, x1, y1);
    } else {
        plot_line_high(c, col, x1, y1, x2, y2);
    }
}

#[wasm_bindgen]
/// General-purpose line drawing function.  Draws a line of a given colour between points (x1,x2)
/// and (y1,y2) to a [`Canvas`].
///
/// Defers actual drawing to [`h_line`], [`v_line`] or [`line_bresenham`] as appropriate.
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
/// [`h_line`]: ./fn.h_line.html
/// [`v_line`]: ./fn.v_line.html
/// [`line_bresenham`]: ./fn.line_bresenham.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
///   - `col`: colour to use for line
///   - `x1`: x co-ordinate of line start
///   - `y1`: y co-ordinate of line start
///   - `x2`: x co-ordinate of line end
///   - `y2`: y co-ordinate of line end
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::line;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(128, 128);
/// line(&mut c, &ARGBColour::new(255, 255, 0, 0), 10, 20, 110, 120);
/// ```
pub fn line(c: &mut Canvas, col: &ARGBColour, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        v_line(c, col, x1, y1, y2);
    } else if y1 == y2 {
        h_line(c, col, x1, y1, x2);
    } else {
        line_bresenham(c, col, x1, y1, x2, y2);
    }
}
