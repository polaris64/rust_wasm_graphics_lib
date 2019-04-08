use super::lines::h_line;
use crate::canvas::Canvas;
use crate::types::ARGBColour;

pub fn downward_triangle(
    c: &mut Canvas,
    col: &ARGBColour,
    tl_x: isize,
    tl_y: isize,
    tr_x: isize,
    tr_y: isize,
    bot_x: isize,
    bot_y: isize,
) {
    let dxl = (bot_x - tl_x) as f64 / (bot_y - tl_y) as f64;
    let dxr = (bot_x - tr_x) as f64 / (bot_y - tr_y) as f64;
    for y in tl_y..=bot_y {
        let xl = tl_x + ((y - tl_y) as f64 * dxl) as isize;
        let xr = tr_x + ((y - tl_y) as f64 * dxr) as isize;
        h_line(c, col, xl, y, xr);
    }
}

pub fn upward_triangle(
    c: &mut Canvas,
    col: &ARGBColour,
    top_x: isize,
    top_y: isize,
    bl_x: isize,
    bl_y: isize,
    br_x: isize,
    br_y: isize,
) {
    let dxl = (bl_x - top_x) as f64 / (bl_y - top_y) as f64;
    let dxr = (br_x - top_x) as f64 / (br_y - top_y) as f64;
    for y in top_y..=bl_y {
        let xl = bl_x + ((y - bl_y) as f64 * dxl) as isize;
        let xr = br_x + ((y - bl_y) as f64 * dxr) as isize;
        h_line(c, col, xl, y, xr);
    }
}
