use std::fs::File;
use std::io::BufReader;
use geojson::GeoJson;

pub fn read_geojson(file_path: &str) -> Option<GeoJson> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    Some(GeoJson::from_reader(reader).unwrap())
}