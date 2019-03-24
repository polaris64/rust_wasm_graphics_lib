use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// A 32-bit ARGB colour
pub struct ARGBColour {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[wasm_bindgen]
impl ARGBColour {

    /// Creates a new ARGBColour from ARGB u8 components
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }
}

impl From<&ARGBColour> for u32 {
    fn from(x: &ARGBColour) -> u32 {
        u32::from(x.a) << 24 | u32::from(x.r) << 16 | u32::from(x.g) << 8 | u32::from(x.b)
    }
}
