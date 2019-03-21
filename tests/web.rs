//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

extern crate rust_wasm_graphics_lib;
use crate::rust_wasm_graphics_lib::canvas;
use crate::rust_wasm_graphics_lib::drawing;
use crate::rust_wasm_graphics_lib::types;

#[wasm_bindgen_test]
fn canvas_create() {
    let mut canv = canvas::Canvas::new(10, 20);
    assert_eq!(canv.width(),  10);
    assert_eq!(canv.height(), 20);
    assert_eq!(canv.buffer().len(), 10 * 20);
}

#[wasm_bindgen_test]
fn canvas_buffer_index() {
    let canv = canvas::Canvas::new(10, 20);
    assert_eq!(canv.buffer_index(5, 6), 6 * 10 + 5);
    assert_eq!(canv.buffer_index(7, 8), 8 * 10 + 7);
    let canv = canvas::Canvas::new(15, 28);
    assert_eq!(canv.buffer_index(5, 6), 6 * 15 + 5);
    assert_eq!(canv.buffer_index(7, 8), 8 * 15 + 7);
}

#[wasm_bindgen_test]
fn canvas_clear() {
    let mut canv = canvas::Canvas::new(3, 4);
    let col = types::ARGBColour::new(255, 255, 0, 0);
    let col_u32 = u32::from(&col);

    // Assert that no canvas pixels have "col" value
    assert_eq!(canv.buffer().iter().any(|x| *x == col_u32), false);

    // Clear the canvas to "col"
    canv.clear(&col);

    // Assert that all canvas pixels now have "col" value
    assert_eq!(canv.buffer().iter().all(|x| *x == col_u32), true);
}

#[wasm_bindgen_test]
fn drawing_fill_rect() {
    let mut canv = canvas::Canvas::new(4, 6);
    let col = types::ARGBColour::new(255, 255, 0, 0);
    let col_u32 = u32::from(&col);

    // Assert that no canvas pixels have "col" value
    assert_eq!(canv.buffer().iter().any(|x| *x == col_u32), false);

    // Draw a rectangle (2, 4, 5, 7): should fill pixels (2,4), (3,4), (2,5), (3,5)
    drawing::fill_rect(&mut canv, &col, 2, 4, 5, 7);

    // Locations which should not be filled
    let ne_idx = vec![
        (0, 0), (1, 0), (2, 0), (3, 0), 
        (0, 1), (1, 1), (2, 1), (3, 1), 
        (0, 2), (1, 2), (2, 2), (3, 2), 
        (0, 3), (1, 3), (2, 3), (3, 3), 
        (0, 4), (1, 4),
        (0, 5), (1, 5),
    ];

    // Locations which should be filled
    let eq_idx = vec![
        (2, 4), (3, 4), 
        (2, 5), (3, 5), 
    ];

    // Assert all un-filled locations are not set to "col"
    assert_eq!(
        ne_idx.iter().all(|x| {
            let idx = canv.buffer_index(x.0, x.1);
            canv.buffer()[idx] != col_u32
        }),
        true
    );

    // Assert all filled locations are set to "col"
    assert_eq!(
        eq_idx.iter().all(|x| {
            let idx = canv.buffer_index(x.0, x.1);
            canv.buffer()[idx] == col_u32
        }),
        true
    );
}
