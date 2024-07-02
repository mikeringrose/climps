use geojson::GeoJson;
use crate::map::Map;

pub trait Renderer {
    fn render(self: &Self, map: &Map, geojson: &GeoJson);
}

pub struct AsciiRenderer {
    pub width: usize,
    pub height: usize,
}

impl AsciiRenderer {
    fn map_to_screen_coords(self: &Self, map: &Map, x: f64, y: f64) -> Option<(usize, usize)> {
        let (min_xy, max_xy) = map.calculate_viewport();

        if x < min_xy.0 || x > max_xy.0 || y < min_xy.1 || y > max_xy.1 {
            return None;
        }
    
        let screen_x = ((x - min_xy.0) / (max_xy.0 - min_xy.0) * self.width as f64) as usize;
        let screen_y = self.height - 1 - ((y - min_xy.1) / (max_xy.1 - min_xy.1) * self.height as f64) as usize;
    
        if screen_x < self.width && screen_y < self.height {
            Some((screen_x, screen_y))
        } else {
            None
        }
    }

    fn draw_line(self: &Self, x0: usize, y0: usize, x1: usize, y1: usize, buffer: &mut Vec<Vec<char>>) {
        let dx = isize::abs(x1 as isize - x0 as isize);
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -isize::abs(y1 as isize - y0 as isize);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
    
        let mut x = x0 as isize;
        let mut y = y0 as isize;
    
        loop {
            if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                buffer[y as usize][x as usize] = '+';
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

    fn render_ascii(self: &Self, map: &Map, geojson: &GeoJson, buffer: &mut Vec<Vec<char>>) {
        if let GeoJson::FeatureCollection(collection) = geojson {
            for feature in &collection.features {
                if let Some(geometry) = &feature.geometry {
                    match geometry.value {
                        geojson::Value::Point(ref point) => {
                            if let Some((x, y)) = self.map_to_screen_coords(map, point[0], point[1]) {
                                buffer[y][x] = '*';
                            }
                        },
                        geojson::Value::LineString(ref line) => {
                            for coord in line {
                                if let Some((x, y)) = self.map_to_screen_coords(map, coord[0], coord[1]) {
                                    buffer[y][x] = '#';
                                }
                            }
                        },
                        geojson::Value::Polygon(ref polygons) => {
                            for polygon in polygons {
                                for i in 0..polygon.len() {
                                    let start = &polygon[i];
                                    let end = &polygon[(i + 1) % polygon.len()];
                                    if let (Some((x0, y0)), Some((x1, y1))) = (
                                        self.map_to_screen_coords(map, start[0], start[1]),
                                        self.map_to_screen_coords(map, end[0], end[1]),
                                    ) {
                                        self.draw_line(x0, y0, x1, y1, buffer);
                                    }
                                }
                            }
                        },
                        geojson::Value::MultiPolygon(ref multipolygons) => {
                            for multipolygon in multipolygons {
                                for polygon in multipolygon {
                                    for i in 0..polygon.len() {
                                        let start = &polygon[i];
                                        let end = &polygon[(i + 1) % polygon.len()];
                                        if let (Some((x0, y0)), Some((x1, y1))) = (
                                            self.map_to_screen_coords(map, start[0], start[1]),
                                            self.map_to_screen_coords(map, end[0], end[1]),
                                        ) {
                                            self.draw_line(x0, y0, x1, y1, buffer);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl Renderer for AsciiRenderer {
    fn render(self: &Self, map: &Map, geojson: &GeoJson) {
        let mut buffer = vec![vec!['.'; self.width]; self.height];

        self.render_ascii(map, geojson, &mut buffer);

        for row in buffer {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

// use std::io::{Stdout, Write};
// use crossterm::cursor::MoveTo;
// use crossterm::terminal::{Clear, ClearType};
// use crossterm::QueueableCommand;
// use crossterm::style::{SetBackgroundColor, Color};
// 
// pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
//     if force {
//         stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
//         stdout.queue(Clear(ClearType::All)).unwrap();
//         stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
//     }

//     for (x, col) in curr_frame.iter().enumerate() {
//         for (y, s) in col.iter().enumerate() {
//             if *s != last_frame[x][y] || force {
//                 stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
//                 println!("{}", *s);
//             }
//         }
//     }
//     stdout.flush().unwrap();
// }
