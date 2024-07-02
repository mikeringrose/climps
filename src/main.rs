use std::thread;
use std::time::Duration;

use climps::map::Map;
use climps::projection::Mercator;
use climps::reader::read_geojson;
use climps::render::{AsciiRenderer, Renderer};

use crossterm::{event, terminal};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{Clear, ClearType};

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

    renderer.render(&map, &geojson);

    terminal::enable_raw_mode().expect("Could not turn on Raw mode");

    // user input
    'mainloop: loop {
        if event::poll(Duration::from_millis(500)).expect("Error") {
            if let Event::Key(key_event) = event::read().expect("Failed to read line") {
                match key_event.code {
                    KeyCode::Left => {
                        terminal::disable_raw_mode().expect("Unable to disable raw mode");
                        map.center = (map.center.0 - 1.0, map.center.1);
                        renderer.render(&map, &geojson);
                        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
                    }
                    KeyCode::Up => {
                        terminal::disable_raw_mode().expect("Unable to disable raw mode");
                        map.center = (map.center.0, map.center.1 + 1.0);
                        renderer.render(&map, &geojson);
                        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
                    }
                    KeyCode::Char('z') => {
                        terminal::disable_raw_mode().expect("Unable to disable raw mode");
                        map.zoom += 1.0;
                        renderer.render(&map, &geojson);
                        terminal::enable_raw_mode().expect("Could not turn on Raw mode");
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        terminal::disable_raw_mode().expect("Unable to disable raw mode");
                        break 'mainloop;
                    }
                    _ => {
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(1));
    }
}
