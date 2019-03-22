import {
  ARGBColour,
  Canvas,
  fill_rect,
  h_line,
  v_line,
} from 'rust-wasm-graphics-lib';

import { memory } from 'rust-wasm-graphics-lib/rust_wasm_graphics_lib_bg';

const SIZE_MULT = 4;
const clear_colour = ARGBColour.new(255, 0, 128, 0);
const draw_colour = ARGBColour.new(255, 0, 0, 0);

const rust_canvas = Canvas.new(128, 128);
const height      = rust_canvas.height();
const width       = rust_canvas.width();

const canvas = document.getElementById('main-canvas');
canvas.height = height * SIZE_MULT;
canvas.width  = width  * SIZE_MULT;

const canvas_back = document.createElement('canvas');
canvas_back.height = height;
canvas_back.width  = width;

const ctx = canvas.getContext('2d');
const ctx_back = canvas_back.getContext('2d');
ctx.imageSmoothingEnabled = false;

let counter = 0;

const demo_colourful_rect = () => {
  let nr, ng, nb;

  nr = Math.floor(((Math.sin(counter * 1.5) + 1) / 2) * 255);
  ng = Math.floor(((Math.cos(counter * 1.7) + 1) / 2) * 255);
  nb = Math.floor(((Math.sin(counter * 1.3) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let rectx = Math.floor(((Math.sin(counter * 1.3) + 1) / 2) * width  / 2);
  let recty = Math.floor(((Math.cos(counter * 1.5) + 1) / 2) * height / 2);
  let rectw = Math.floor(((Math.sin(counter * 2.1) + 1) / 2) * width  / 2) + 16;
  let recth = Math.floor(((Math.cos(counter * 1.8) + 1) / 2) * height / 2) + 16;

  fill_rect(
    rust_canvas,
    draw_colour,
    rectx,
    recty,
    rectx + rectw,
    recty + recth,
  );

};

const demo_hlines = () => {
  let nr = Math.floor(((Math.sin(counter * 1.1) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.8) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 1.4) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let x1 = Math.floor(((Math.sin(counter * 0.3) + 1) / 2) * width);
  let x2 = Math.floor(((Math.sin(counter * 1.1) + 1) / 2) * width) + 16;
  let y = Math.floor(((Math.cos(counter * 0.5) + 1) / 2) * height);

  h_line(rust_canvas, draw_colour, x1, y, x2);
};

const demo_vlines = () => {
  let nr = Math.floor(((Math.sin(counter * 1.2) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 1.3) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 1.8) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let y1 = Math.floor(((Math.sin(counter * 0.5) + 1) / 2) * height);
  let y2 = Math.floor(((Math.sin(counter * 1.2) + 1) / 2) * height) + 16;
  let x = Math.floor(((Math.cos(counter * 0.3) + 1) / 2) * width);

  v_line(rust_canvas, draw_colour, x, y1, y2);
};

const renderLoop = () => {
  //debugger;

  if (counter > Math.PI * 10) {
    let nr = Math.floor(((Math.sin(counter * 10.2) + 1) / 2) * 255);
    let ng = Math.floor(((Math.cos(counter * 10.3) + 1) / 2) * 255);
    let nb = Math.floor(((Math.sin(counter * 10.1) + 1) / 2) * 255);
    clear_colour.r = nr;
    clear_colour.g = ng;
    clear_colour.b = nb;

    rust_canvas.clear(clear_colour);
    counter = 0;
  }

  demo_colourful_rect();
  demo_hlines();
  demo_vlines();

  drawBuffer();
  counter += 0.025;

  requestAnimationFrame(renderLoop);
};

const drawBuffer = () => {
  const buffer = rust_canvas.buffer_ptr();
  const buff_u8 = new Uint8ClampedArray(memory.buffer, buffer, width * height * 4);
  const img = new ImageData(buff_u8, width, height);
  ctx_back.putImageData(img, 0, 0);
  ctx.drawImage(canvas_back, 0, 0, width, height, 0, 0, width * SIZE_MULT, height * SIZE_MULT);
};

renderLoop();
