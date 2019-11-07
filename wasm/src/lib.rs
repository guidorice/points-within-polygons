use geojson::{FeatureCollection, GeoJson};
use points_within_polygons::points_within_polygons as lib_points_within_polygons;
use wasm_bindgen::prelude::*;

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Helper function to extract a FeatureCollection and panic if the GeoJson
/// contains anything else.
fn feature_collection(gj: GeoJson) -> FeatureCollection {
    match gj {
        GeoJson::FeatureCollection(fc) => fc,
        _ => {
            panic!("GeoJson must contain a FeatureCollection");
        }
    }
}

#[wasm_bindgen]
pub fn points_within_polygons(points: JsValue, polygons: JsValue) -> JsValue {
    utils::set_panic_hook();
    let points_geojson: GeoJson = points.into_serde().unwrap();
    let polygons_geojson: GeoJson = polygons.into_serde().unwrap();
    let points_fc = feature_collection(points_geojson);
    let polygons_fc = feature_collection(polygons_geojson);
    let opt_feature_collection = lib_points_within_polygons(points_fc, polygons_fc);
    match opt_feature_collection {
        Some(feature_collection) => JsValue::from_serde(&feature_collection).unwrap(),
        None => JsValue::from_bool(false),
    }
}
