use crate::{BoundingBox, LngLat, Point, EARTH_RADIUS};

pub trait Projection {
    fn get_map_bounds(self: &Self) -> BoundingBox;
    fn project(self: &Self, coordinate: LngLat) -> Point;
}

pub struct Mercator;

impl Projection for Mercator {

    fn get_map_bounds(self: &Self) -> BoundingBox {
        ((-180.0, -85.05112878), (180.0, 85.05112878))
    }

    fn project(self: &Self, coordinate: LngLat) -> Point {
        let x = coordinate.0.to_radians() * EARTH_RADIUS;
        let y = (coordinate.1.to_radians().tan() + 1.0 / (coordinate.1.to_radians().cos())).ln() * EARTH_RADIUS;
        (x, y)        
    }
}