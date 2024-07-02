pub mod frame;
pub mod render;
pub mod reader;
pub mod projection;
pub mod map;

pub const WIDTH: usize = 140;
pub const HEIGHT: usize = 60;

pub const EARTH_RADIUS: f64 = 6378137.0;

pub type LngLat = (f64, f64);
pub type Point = (f64, f64);
pub type BoundingBox = (LngLat, LngLat);
