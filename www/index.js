import {
  ARGBColour,
  Canvas,
  fill_rect,
} from 'rust-wasm-graphics-lib';

import { memory } from 'rust-wasm-graphics-lib/rust_wasm_graphics_lib_bg';

const SIZE_MULT = 4;
const clear_colour = ARGBColour.new(255, 0, 128, 0);
const rect_colour = ARGBColour.new(255, 0, 0, 0);

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
  rect_colour.r = nr;
  rect_colour.g = ng;
  rect_colour.b = nb;

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

  let rectx = Math.floor(((Math.sin(counter * 1.3) + 1) / 2) * width  / 2);
  let recty = Math.floor(((Math.cos(counter * 1.5) + 1) / 2) * height / 2);
  let rectw = Math.floor(((Math.sin(counter * 2.1) + 1) / 2) * width  / 2) + 16;
  let recth = Math.floor(((Math.cos(counter * 1.8) + 1) / 2) * height / 2) + 16;

  fill_rect(
    rust_canvas,
    rect_colour,
    rectx,
    recty,
    rectx + rectw,
    recty + recth,
  );

};

const renderLoop = () => {
  //debugger;

  demo_colourful_rect();

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
