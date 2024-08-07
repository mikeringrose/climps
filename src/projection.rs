use std::f64::consts::PI;

use crate::{BoundingBox, LngLat, Point, EARTH_RADIUS};

pub trait Projection {
    fn get_map_bounds(self: &Self) -> BoundingBox;
    fn project(self: &Self, coordinate: LngLat) -> Point;
    fn unproject(self: &Self, point: Point) -> LngLat;
}

pub struct Mercator;

impl Projection for Mercator {

    fn get_map_bounds(self: &Self) -> BoundingBox {
        ((-180.0, -85.05112878), (180.0, 85.05112878))
    }

    fn project(self: &Self, coordinate: LngLat) -> Point {
        let extent_x = EARTH_RADIUS * PI;
        let meters_per = extent_x / PI;
        let lat_rad = coordinate.1.to_radians();
        let x = coordinate.0.to_radians() * meters_per;
        let y = (lat_rad.tan() + (1.0 / lat_rad.cos())).ln() * meters_per;
        (x, y)
    }

    fn unproject(self: &Self, point: Point) -> LngLat {
        let lng = (point.0 / EARTH_RADIUS) * (180.0 / PI as f64);
        let lat = (point.1 / EARTH_RADIUS).exp();
        (lng, lat)
    }
}