use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;
use crate::types::ARGBColour;

#[wasm_bindgen]
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
    if x2 > c.width() { x2 = c.width() }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x1, y);
    for _ in x1..x2 {
        c.buffer()[idx] = col;
        idx += 1;
    }
}

#[wasm_bindgen]
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
    if y2 > c.height() { y2 = c.height() }
    let col: u32 = col.into();
    let mut idx = c.buffer_index(x, y1);
    for _ in y1..y2 {
        c.buffer()[idx] = col;
        idx += c.width()
    }
}
