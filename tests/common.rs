use geojson::{FeatureCollection, GeoJson};

/// Helper function to extract a FeatureCollection and panic if the GeoJson
/// contains anything else.
pub fn feature_collection(gj: GeoJson) -> FeatureCollection {
    match gj {
        GeoJson::FeatureCollection(fc) => fc,
        _ => unreachable!(),
    }
}
