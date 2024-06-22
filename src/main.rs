use std::fs::File;
use std::io::BufReader;
use geojson::GeoJson;

const WIDTH: usize = 140;
const HEIGHT: usize = 50;


// Define the bounds of your coordinate system
// const MIN_X: f64 = -23.770313590783616; // Minimum longitude
// const MAX_X: f64 = 73.76449336498158;  // Maximum longitude
// const MIN_Y: f64 = -36.210619428235965;  // Minimum latitude
// const MAX_Y: f64 = 37.45391665555519;   // Maximum latitude

const MIN_X: f64 = -125.0; // Minimum longitude
const MAX_X: f64 = -66.93457;  // Maximum longitude
const MIN_Y: f64 = 24.396308;  // Minimum latitude
const MAX_Y: f64 = 49.384358;   // Maximum latitude

fn read_geojson(file_path: &str) -> GeoJson {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    GeoJson::from_reader(reader).expect("Error parsing GeoJSON")
}

fn map_to_screen_coords(x: f64, y: f64) -> Option<(usize, usize)> {
    if x < MIN_X || x > MAX_X || y < MIN_Y || y > MAX_Y {
        return None;
    }

    let screen_x = ((x - MIN_X) / (MAX_X - MIN_X) * WIDTH as f64) as usize;
    let screen_y = HEIGHT - 1 - ((y - MIN_Y) / (MAX_Y - MIN_Y) * HEIGHT as f64) as usize;

    if screen_x < WIDTH && screen_y < HEIGHT {
        Some((screen_x, screen_y))
    } else {
        None
    }
}

fn draw_line(x0: usize, y0: usize, x1: usize, y1: usize, map: &mut Vec<Vec<char>>) {
    let dx = isize::abs(x1 as isize - x0 as isize);
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -isize::abs(y1 as isize - y0 as isize);
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0 as isize;
    let mut y = y0 as isize;

    loop {
        if x >= 0 && x < WIDTH as isize && y >= 0 && y < HEIGHT as isize {
            map[y as usize][x as usize] = '+';
        }
        if x == x1 as isize && y == y1 as isize { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn render_ascii(geojson: &GeoJson, map: &mut Vec<Vec<char>>) {
    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in &collection.features {
            if let Some(geometry) = &feature.geometry {
                match geometry.value {
                    geojson::Value::Point(ref point) => {
                        if let Some((x, y)) = map_to_screen_coords(point[0], point[1]) {
                            map[y][x] = '*';
                        }
                    },
                    geojson::Value::LineString(ref line) => {
                        for coord in line {
                            if let Some((x, y)) = map_to_screen_coords(coord[0], coord[1]) {
                                map[y][x] = '#';
                            }
                        }
                    },
                    geojson::Value::Polygon(ref polygons) => {
                        for polygon in polygons {
                            for i in 0..polygon.len() {
                                let start = &polygon[i];
                                let end = &polygon[(i + 1) % polygon.len()];
                                if let (Some((x0, y0)), Some((x1, y1))) = (
                                    map_to_screen_coords(start[0], start[1]),
                                    map_to_screen_coords(end[0], end[1]),
                                ) {
                                    draw_line(x0, y0, x1, y1, map);
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn main() {
    let file_path = "/Users/mikeringrose/Projects/geojson_ascii_viewer/data/us.geojson";
    let geojson = read_geojson(file_path);

    let mut map = vec![vec!['.'; WIDTH]; HEIGHT];

    render_ascii(&geojson, &mut map);

    print_map(&map);
}