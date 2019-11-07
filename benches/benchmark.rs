#![feature(test)]

extern crate points_within_polygons;
extern crate test;

#[cfg(test)]
mod tests {
    use geojson::{FeatureCollection, GeoJson};
    use points_within_polygons::points_within_polygons;
    use std::fs;
    use test::Bencher;

    fn feature_collection(gj: GeoJson) -> FeatureCollection {
        match gj {
            GeoJson::FeatureCollection(fc) => fc,
            _ => unreachable!(),
        }
    }

    #[bench]
    fn bench_points_within_polygons_simple_polygons_10points(b: &mut Bencher) {
        let fixtures_path = "tests/fixtures/natural-earth/";
        let expect_fail_msg = "Unable to read file";

        let points_data = fs::read_to_string([fixtures_path, "points-10.geojson"].concat())
            .expect(expect_fail_msg);
        let points_geojson = points_data.parse::<GeoJson>().unwrap();
        let points_fc = feature_collection(points_geojson);

        let polygons_data = fs::read_to_string([fixtures_path, "ne_110m_land.geojson"].concat())
            .expect(expect_fail_msg);
        let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
        let polygons_fc = feature_collection(polygons_geojson);
        b.iter(|| {
            // TODO: clone() is making benchmark irrellevant
            points_within_polygons(points_fc.clone(), polygons_fc.clone());
        });
    }

    #[bench]
    fn bench_points_within_polygons_simple_polygons_100points(b: &mut Bencher) {
        let fixtures_path = "tests/fixtures/natural-earth/";
        let expect_fail_msg = "Unable to read file";

        let points_data = fs::read_to_string([fixtures_path, "points-100.geojson"].concat())
            .expect(expect_fail_msg);
        let points_geojson = points_data.parse::<GeoJson>().unwrap();
        let points_fc = feature_collection(points_geojson);

        let polygons_data = fs::read_to_string([fixtures_path, "ne_110m_land.geojson"].concat())
            .expect(expect_fail_msg);
        let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
        let polygons_fc = feature_collection(polygons_geojson);

        b.iter(|| {
            // TODO: clone() is making benchmark irrellevant
            points_within_polygons(points_fc.clone(), polygons_fc.clone());
        });
    }

    #[bench]
    fn bench_points_within_polygons_complex_polygons_10points(b: &mut Bencher) {
        let fixtures_path = "tests/fixtures/natural-earth/";
        let expect_fail_msg = "Unable to read file";

        let points_data = fs::read_to_string([fixtures_path, "points-10.geojson"].concat())
            .expect(expect_fail_msg);
        let points_geojson = points_data.parse::<GeoJson>().unwrap();
        let points_fc = feature_collection(points_geojson);

        let polygons_data = fs::read_to_string([fixtures_path, "ne_10m_land.geojson"].concat())
            .expect(expect_fail_msg);
        let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
        let polygons_fc = feature_collection(polygons_geojson);

        b.iter(|| {
            // TODO: clone() is making benchmark irrellevant
            points_within_polygons(points_fc.clone(), polygons_fc.clone());
        });
    }

    #[bench]
    fn bench_points_within_polygons_complex_polygons_100points(b: &mut Bencher) {
        let fixtures_path = "tests/fixtures/natural-earth/";
        let expect_fail_msg = "Unable to read file";

        let points_data = fs::read_to_string([fixtures_path, "points-100.geojson"].concat())
            .expect(expect_fail_msg);
        let points_geojson = points_data.parse::<GeoJson>().unwrap();
        let points_fc = feature_collection(points_geojson);

        let polygons_data = fs::read_to_string([fixtures_path, "ne_10m_land.geojson"].concat())
            .expect(expect_fail_msg);
        let polygons_geojson = polygons_data.parse::<GeoJson>().unwrap();
        let polygons_fc = feature_collection(polygons_geojson);

        b.iter(|| {
            // TODO: clone() is making benchmark irrellevant
            points_within_polygons(points_fc.clone(), polygons_fc.clone());
        });
    }
}
