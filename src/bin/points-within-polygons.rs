use clap::{App, Arg};
use geojson::{FeatureCollection, GeoJson};
use points_within_polygons::points_within_polygons;
use std::fs;

/// Helper function to extract a FeatureCollection and panic if the GeoJson
/// contains anything else.
pub fn feature_collection(gj: GeoJson) -> FeatureCollection {
    match gj {
        GeoJson::FeatureCollection(fc) => fc,
        _ => panic!("GeoJson must contain a FeatureCollection"),
    }
}

fn main() {
    let param_matches = App::new("points-within-polygons")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("points")
                .long("points")
                .value_name("FILE")
                .help("Filename containing a geojson of FeatureCollection of Points")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("polygons")
                .long("polygons")
                .value_name("FILE")
                .help("Filename containing a geojson of FeatureCollection of Polygons or MultiPolygons")
                .takes_value(true)
                .required(true)
        )
        .get_matches();
    let points_filename = param_matches.value_of("points").unwrap();
    let polygons_filename = param_matches.value_of("polygons").unwrap();
    let points_data = fs::read_to_string(points_filename).expect("failed to read points data file");
    let points_geojson = points_data.parse::<GeoJson>().unwrap();
    let points_fc = feature_collection(points_geojson);
    let polygons_data =
        fs::read_to_string(polygons_filename).expect("failed to read polygons data file");
    let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
    let polygons_fc = feature_collection(polygons_geojson);

    let result_fc = points_within_polygons(points_fc, polygons_fc);
    match result_fc {
        Some(fc) => {
            let geojson = GeoJson::FeatureCollection(fc);
            println!("{}", geojson.to_string());
        }
        None => println!("# no points are contained in the polygons"),
    }
}
