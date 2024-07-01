pub type LngLat = (f64, f64);
pub type Point = (f64, f64);

pub const EARTH_RADIUS: f64 = 6378137.0;

pub trait Projection {
    fn project(self: &Self, coordinate: LngLat) -> Point;
    // fn unproject(point: Point) -> LngLat; 
}

pub struct Mercator;

impl Projection for Mercator {
    fn project(self: &Self, coordinate: LngLat) -> Point {
        let x = coordinate.0.to_radians() * EARTH_RADIUS;
        let y = (coordinate.1.to_radians().tan() + 1.0 / (coordinate.1.to_radians().cos())).ln() * EARTH_RADIUS;
        (x, y)        
    }

    // fn unproject(point: Point) -> LngLat {
    //     todo!()
    // }
}