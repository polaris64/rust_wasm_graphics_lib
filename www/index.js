import {
  ARGBColour,
  Canvas,
  fill_polygon,
  fill_rect,
  fill_triangle,
  line,
  polygon,
  rect,
} from 'rust-wasm-graphics-lib';

import { memory } from 'rust-wasm-graphics-lib/rust_wasm_graphics_lib_bg';

const SIZE_MULT = 2;
const WIDTH = 256;
const HEIGHT = 256;

const POLY_POINTS = [
  [1.0, 0.0],
  [-0.8090169943749473, 0.5877852522924732],
  [0.30901699437494723, -0.9510565162951536],
  [0.30901699437494745, 0.9510565162951535],
  [-0.8090169943749476, -0.587785252292473],
  [1.0, 0.0],
];
/*
const POLY_POINTS_TRI = [
  [ 0, -1],
  [ 1,  1],
  [-1,  1],
];
*/
const POLY_SIZE = WIDTH / 3;

const clear_colour = ARGBColour.new(255, 0, 128, 0);
const draw_colour = ARGBColour.new(64, 0, 0, 0);

const rust_canvas = Canvas.new(WIDTH, HEIGHT);
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

const demo_rect = () => {
  let nr, ng, nb;

  nr = Math.floor(((Math.cos(counter * 1.1) + 1) / 2) * 255);
  ng = Math.floor(((Math.sin(counter * 1.4) + 1) / 2) * 255);
  nb = Math.floor(((Math.cos(counter * 2.3) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let rectx = Math.floor(((Math.cos(counter * 1.8) + 1) / 2) * width  / 2);
  let recty = Math.floor(((Math.sin(counter * 2.2) + 1) / 2) * height / 2);
  let rectw = Math.floor(((Math.cos(counter * 1.1) + 1) / 2) * width  / 2) + 16;
  let recth = Math.floor(((Math.sin(counter * 1.4) + 1) / 2) * height / 2) + 16;

  rect(
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

  line(rust_canvas, draw_colour, x1, y, x2, y);
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

  line(rust_canvas, draw_colour, x, y1, x, y2);
};

const demo_lines = () => {
  let nr = Math.floor(((Math.sin(counter * 1.8) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.2) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 0.7) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let x1 = Math.floor(((Math.sin(counter * 0.8) + 1) / 2) * width);
  let x2 = Math.floor(((Math.cos(counter * 1.1) + 1) / 2) * width) + 16;
  let y1 = Math.floor(((Math.sin(counter * 0.5) + 1) / 2) * height);
  let y2 = Math.floor(((Math.cos(counter * 1.2) + 1) / 2) * height) + 16;

  line(rust_canvas, draw_colour, x1, y1, x2, y2);
};

const demo_rotating_line = () => {
  let nr = Math.floor(((Math.sin(counter * 1.4) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.6) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 0.9) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  let x1 = WIDTH / 2;
  let y1 = HEIGHT / 2;
  let x2 = (WIDTH / 2) + (100 * Math.cos(-counter));
  let y2 = (HEIGHT / 2) + (100 * Math.sin(-counter));

  line(rust_canvas, draw_colour, x1, y1, x2, y2);
};

const demo_polygon = () => {
  let nr = Math.floor(((Math.sin(counter * 1.4) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.6) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 0.9) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  const points = POLY_POINTS
    .map((pt) => [
      ((pt[0] * POLY_SIZE * Math.cos(counter)) - (pt[1] * POLY_SIZE * Math.sin(counter))) + (WIDTH  / 2),
      ((pt[0] * POLY_SIZE * Math.sin(counter)) + (pt[1] * POLY_SIZE * Math.cos(counter))) + (HEIGHT / 2),
    ])
    .reduce((a, pt) => a.concat(pt), []);

  polygon(
    rust_canvas,
    draw_colour,
    false,
    points,
  );
};

const demo_fill_polygon = () => {
  let nr = Math.floor(((Math.sin(counter * 0.7) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.5) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 0.2) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  const points = POLY_POINTS
    .map((pt) => [
      ((pt[0] * POLY_SIZE * Math.cos(counter)) - (pt[1] * POLY_SIZE * Math.sin(counter))) + (WIDTH  / 2),
      ((pt[0] * POLY_SIZE * Math.sin(counter)) + (pt[1] * POLY_SIZE * Math.cos(counter))) + (HEIGHT / 2),
    ])
    .reduce((a, pt) => a.concat(pt), []);

  fill_polygon(
    rust_canvas,
    draw_colour,
    points,
  );
};

const demo_fill_triangle = () => {
  let nr = Math.floor(((Math.sin(counter * 0.7) + 1) / 2) * 255);
  let ng = Math.floor(((Math.cos(counter * 0.5) + 1) / 2) * 255);
  let nb = Math.floor(((Math.sin(counter * 0.2) + 1) / 2) * 255);
  draw_colour.r = nr;
  draw_colour.g = ng;
  draw_colour.b = nb;

  // Rotating
  /*
  const pts = POLY_POINTS_TRI
    .map((pt) => [
      ((pt[0] * POLY_SIZE / 2 * Math.cos(counter)) - (pt[1] * POLY_SIZE / 2 * Math.sin(counter))) + (WIDTH  / 2),
      ((pt[0] * POLY_SIZE / 2 * Math.sin(counter)) + (pt[1] * POLY_SIZE / 2 * Math.cos(counter))) + (HEIGHT / 2),
    ])
    .reduce((a, pt) => a.concat(pt), []);

  fill_triangle(
    rust_canvas,
    draw_colour,
    ...pts
  );
  */

  // Flying vertices
  fill_triangle(
    rust_canvas,
    draw_colour,
    Math.floor(((Math.cos(counter * 1.2) + 1) / 2) * WIDTH),
    Math.floor(((Math.sin(counter * 1.5) + 1) / 2) * HEIGHT),
    Math.floor(((Math.cos(counter * 1.5) + 1) / 2) * WIDTH),
    Math.floor(((Math.sin(counter * 1.2) + 1) / 2) * HEIGHT),
    Math.floor(((Math.cos(counter * 1.9) + 1) / 2) * WIDTH),
    Math.floor(((Math.sin(counter * 1.7) + 1) / 2) * HEIGHT),
  );
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
  demo_rect();
  demo_hlines();
  demo_vlines();
  demo_rotating_line();
  demo_lines();
  demo_fill_polygon();
  demo_polygon();
  demo_fill_triangle();

  drawBuffer();
  counter += 0.01;

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
