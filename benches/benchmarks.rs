use criterion::{Criterion, criterion_group, criterion_main};

use rust_wasm_graphics_lib::canvas::Canvas;
use rust_wasm_graphics_lib::drawing::{fill_rect};
use rust_wasm_graphics_lib::types::ARGBColour;

fn bench_canvas_clear(c: &mut Criterion) {
    let col = ARGBColour::new(255, 255, 0, 0);
    let mut can = Canvas::new(128, 128);
    c.bench_function("Canvas::clear()", move |b| b.iter(|| can.clear(&col)));
}

fn bench_drawing_fill_rect(c: &mut Criterion) {
    let col = ARGBColour::new(255, 255, 0, 0);
    let mut can = Canvas::new(128, 128);
    c.bench_function(
        "drawing::fill_rect()",
        move |b| b.iter(|| fill_rect(&mut can, &col, 16, 16, 112, 112))
    );
}
criterion_group!(benches,
    bench_canvas_clear,
    bench_drawing_fill_rect,
);

criterion_main!(benches);
