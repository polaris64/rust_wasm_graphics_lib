use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;
use crate::types::ARGBColour;
use super::lines::{h_line, line};

#[wasm_bindgen]
/// Draws an un-filled polygon given a list of vertices with a given stroke colour.
///
/// Vertices should be listed as a flat array of x,y co-ordinates, therefore the length should
/// always be a multiple of 2.
///
/// If the `close` flag is set, a line will also be drawn from the last vertex to the first to
/// close the polygon.
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
///   - `col`: colour to use for line
///   - `close`: if set, will close the polygon by drawing a line from the last vertex to the
///      first.
///   - `points`: flat list of vertices with components in x,y order
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::shape::polygon;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(128, 128);
///
/// // Draw a triangle
/// polygon(&mut c, &ARGBColour::new(255, 255, 0, 0), true, vec![0, -10, 10, 10, -10, 10]);
/// ```
pub fn polygon(c: &mut Canvas, col: &ARGBColour, close: bool, points: Vec<i32>) {
    points.as_slice().chunks(2).zip(points.as_slice().chunks(2).skip(1)).for_each(|(curr, next)| {
        line(c, col, curr[0] as isize, curr[1] as isize, next[0] as isize, next[1] as isize);
    });
    if close && points.len() >= 4 {
        let fx = points[0];
        let fy = points[1];
        let lx = points[points.len() - 2];
        let ly = points[points.len() - 1];
        line(c, col, lx as isize, ly as isize, fx as isize, fy as isize);
    }
}

#[wasm_bindgen]
/// Draws a filled polygon given a list of vertices with a given fill colour.
///
/// Note that the polygon should be closed, meaning that the last vertex should match the first, in
/// order for the routine to work correctly.
///
/// See [`polygon()`] for further details.
///
/// [`Canvas`]: ../canvas/struct.Canvas.html
/// [`polygon()`]: ./fn.polygon.html
///
/// # Arguments:
///
///   - `c`: target [`Canvas`]
///   - `col`: colour to use for line
///   - `close`: if set, will close the polygon by drawing a line from the last vertex to the
///      first.
///   - `points`: flat list of vertices with components in x,y order
///
/// # Example:
///
/// ```
/// use rust_wasm_graphics_lib::canvas::Canvas;
/// use rust_wasm_graphics_lib::drawing::shape::fill_polygon;
/// use rust_wasm_graphics_lib::types::ARGBColour;
///
/// let mut c = Canvas::new(128, 128);
///
/// // Draw a filled triangle
/// fill_polygon(&mut c, &ARGBColour::new(255, 255, 0, 0), vec![0, -10, 10, 10, -10, 10, 0, -10]);
/// ```
pub fn fill_polygon(c: &mut Canvas, col: &ARGBColour, points: Vec<i32>) {

    // TODO: optimise!

    let ymin = points.as_slice().chunks(2).map(|x| x[1]).min().unwrap_or(0);
    let ymax = points.as_slice().chunks(2).map(|x| x[1]).max().unwrap_or(0);

    for y in ymin..=ymax {

        // Get all edges as a vector of start/end vertices (each vertex is (x, y)).
        let mut xs: Vec<i32> = points
            .as_slice()
            .chunks(2)
            .zip(points.as_slice().chunks(2).skip(1))
            .map(|(s, e)| ((s[0], s[1]), (e[0], e[1])))

            // Get x co-ordinate of each edge at y
            .map(|e| {
                let miny = if (e.1).1 < (e.0).1 { (e.1).1 } else { (e.0).1 };
                let maxy = if (e.1).1 > (e.0).1 { (e.1).1 } else { (e.0).1 };
                let sx = (e.0).0;
                let ex = (e.1).0;
                let sy = (e.0).1;
                let ey = (e.1).1;
                if miny == maxy || y < miny || y >= maxy {
                    None
                } else if y == maxy {
                    Some(ex)
                } else {
                    let dy = (y - sy) as f32 / (ey - sy) as f32;
                    Some(sx + ((ex as f32 - sx as f32) * dy) as i32)
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        xs.sort_unstable();

        // Draw h_line between all edge x co-ordinates
        xs.as_slice().chunks(2).for_each(|x|
            if x.len() == 2 {
                h_line(c, col, x[0] as isize, y as isize, x[1] as isize)
            }
        );

        // Handle all horizontal edges on this scanline
        points
            .as_slice()
            .chunks(2)
            .zip(points.as_slice().chunks(2).skip(1))
            .map(|(s, e)| ((s[0], s[1]), (e[0], e[1])))
            .filter(|e| (e.0).1 == y && (e.0).1 == (e.1).1)
            .for_each(|e| {
                h_line(c, col, (e.0).0 as isize, y as isize, (e.1).0 as isize)
            });
    }
}
