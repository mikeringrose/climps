use climps::map::Map;
use climps::projection::Mercator;
use climps::reader::read_geojson;
use climps::render::{AsciiRenderer, Renderer};

const WIDTH: usize = 200;
const HEIGHT: usize = 60;

fn main() {
    let mut map = Map::new(Box::new(Mercator), (0.0, 0.0), 0.0);
    // let map = Map::new(Box::new(Mercator), (-99.03715177307542, 28.84464690240982), 2.0);

    let file_path = "/Users/mikeringrose/Projects/climps/data/custom.geo.json";
    let geojson = read_geojson(file_path);

    let renderer = AsciiRenderer {
        width: WIDTH,
        height: HEIGHT,
    };

    renderer.render(&mut map, &geojson);
}
