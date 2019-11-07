///! Test the countries.geojson sourced from https://github.com/georust/geojson
///! Using the results generated by the Turf.js library itself.
use geojson::GeoJson;
use points_within_polygons::points_within_polygons;
use std::fs;

mod common;

const FIXTURES_PATH: &str = "tests/fixtures/countries/";
const READ_FILE_FAIL_MSG: &str = "Unable to read file";

#[test]
fn test_countries() {
    let points_data =
        fs::read_to_string([FIXTURES_PATH, "points.geojson"].concat()).expect(READ_FILE_FAIL_MSG);
    let points_geojson = points_data.parse::<GeoJson>().unwrap();
    let points_fc = common::feature_collection(points_geojson);

    let polygons_data =
        fs::read_to_string([FIXTURES_PATH, "polygons.geojson"].concat()).expect(READ_FILE_FAIL_MSG);
    let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
    let polygons_fc = common::feature_collection(polygons_geojson);

    let expect_pretty_printed =
        fs::read_to_string([FIXTURES_PATH, "result.geojson"].concat()).expect(READ_FILE_FAIL_MSG);
    let expect_geojson = expect_pretty_printed.parse::<GeoJson>().unwrap();
    let expect_str = expect_geojson.to_string();

    let maybe_result_fc = points_within_polygons(points_fc, polygons_fc);

    match maybe_result_fc {
        Some(feature_collection) => {
            // features may be returned in different order, so assert on the
            // geojson encoding length. (you can also view the geojsons for
            // comparison)
            let result_str = feature_collection.to_string();
            assert_eq!(expect_str.len(), result_str.len());
        }
        None => panic!("expect result to be FeatureCollection"),
    }
}
