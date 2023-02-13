use nannou::color::{Srgb, BLACK, GRAY};

pub type Color = Srgb<u8>;

pub const SCREEN_SIZE: u32 = 700;
pub const SCREEN_HALF: u32 = SCREEN_SIZE / 2;
pub const PIE_RADIUS: f32 = 50.;
pub const PIE_BACKGROUND: Color = GRAY;
pub const PIE_ACCEL: f32 = 0.1;
pub const PIE_SPAWN_RATE: u64 = 45;
pub const PLATE_W: f32 = 100.;
pub const PLATE_H: f32 = 20.;
pub const PLATE_Y: f32 = -(SCREEN_HALF as f32) + (PLATE_H as f32 / 2.);
pub const PLATE_COLOR: Color = BLACK;

pub const DIGITS_LOADED_AT_ONCE: usize = 100;

//remove this once pies are drawn with slices
pub const SLICES_FONT_SIZE: u32 = 75;
