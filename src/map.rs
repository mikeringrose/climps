use crate::{projection::{Mercator, Projection}, BoundingBox, LngLat};

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

    pub fn calculate_viewport(&self) -> BoundingBox {
        let (ul, lr) = self.proj.get_map_bounds();
        let (world_max_lon, world_max_lat) = self.proj.project(lr);
        let (world_min_lon, world_min_lat) = self.proj.project(ul);
        let lat_range = (world_max_lat - world_min_lat) / 2_f64.powf(self.zoom);
        let lon_range = (world_max_lon - world_min_lon) / 2_f64.powf(self.zoom);

        let center = self.proj.project(self.center);
        let min_lat = (center.1 - lat_range / 2.0).max(world_min_lat);
        let max_lat = (center.1 + lat_range / 2.0).min(world_max_lat);
        let min_lon = (center.0 - lon_range / 2.0).max(world_min_lon);
        let max_lon = (center.0 + lon_range / 2.0).min(world_max_lon);

        ((min_lon, min_lat), (max_lon, max_lat))
    }
}