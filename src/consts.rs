pub const ASPECT_RATIO: f32 = 1.0;
pub const IMAGE_WIDTH: usize = 400;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f32 = 1.0;
pub const SAMPLES_PER_PIXEL: i32 = 200;
pub const MAX_DEPTH: u32 = 30;
