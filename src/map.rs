use crate::projection::{LngLat, Mercator, Projection};

// This would ensure the lifetimes of Map and Projection are tied
// pub struct Map<'a> {
//     pub proj: &'a dyn Projection,
// }

// This is an auto pointer implementation that gives ownership to the Map
pub struct Map {
    pub proj: Box<dyn Projection>,
    pub center: LngLat,
    pub zoom: f64
}

impl Default for Map {
    fn default() -> Self {
        Map {
            proj: Box::new(Mercator),
            center: (0.0, 0.0),
            zoom: 0.0
        }
    }
}

impl Map {
    pub fn new(proj: Box<dyn Projection>, center: LngLat, zoom: f64) -> Self {
        Map { proj, center, zoom }
    }
}