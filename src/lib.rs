use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::contains::Contains;
use geo_geojson::conversion::from_feature;
use geo_types::{CoordinateType, Geometry, Point, Rect};
use geojson;
use num_traits::{Float, Num, NumCast};
use std::convert::TryInto;
use std::vec::Vec;

struct GeometryWithBounds<T>
where
  T: CoordinateType,
{
  geometry: Geometry<T>,
  bounding_rect: Option<Rect<T>>,
}

/// Get Polygons and MultiPolygons and their bounding rectangles from a geojson
/// FeatureCollection.
fn geometries_with_bounds(
  feature_collection: geojson::FeatureCollection,
) -> Vec<GeometryWithBounds<f64>> {
  feature_collection
    .features
    .into_iter()
    .flat_map(|feature| {
      // TODO: use bbox if it's provided in geojson feature instead of bounding_rect()
      let geometry_collection = from_feature::<f64>(feature);
      geometry_collection
        .into_iter()
        .map(|geometry| match &geometry {
          Geometry::Polygon(_polygon) => {
            // let bounding_rect = polygon.bounding_rect().unwrap();
            Some(GeometryWithBounds {
              geometry,
              bounding_rect: None,
            })
          }
          Geometry::MultiPolygon(_multi_polygon) => {
            // let bounding_rect = multi_polygon.bounding_rect().unwrap();
            Some(GeometryWithBounds {
              geometry,
              bounding_rect: None,
            })
          }
          _ => None,
        })
    })
    .filter_map(|some_geometry| some_geometry)
    .collect()
}

pub fn point_from_geojson_value<T>(value: geojson::Value) -> Option<Point<T>>
where
  T: Num + NumCast + PartialOrd + Copy + Float,
{
  match value {
    geojson::Value::Point(_) => Some(TryInto::<Point<T>>::try_into(value).unwrap()),
    _ => None,
  }
}

// Helper function copied from geojson/src/conversion.rs
fn create_geo_point<T>(point_type: &geojson::PointType) -> geo_types::Point<T>
where
  T: Float,
{
  geo_types::Point::new(
    T::from(point_type[0]).unwrap(),
    T::from(point_type[1]).unwrap(),
  )
}

/// Return Option<FeatureCollection> having the Points which are contained
/// within the given Polygon(s). The function signature is intended to be similar
/// to http://turfjs.org/docs/#pointsWithinPolygon
pub fn points_within_polygons(
  points_fc: geojson::FeatureCollection,
  polygons_fc: geojson::FeatureCollection,
) -> Option<geojson::FeatureCollection> {
  let polygon_geometries = geometries_with_bounds(polygons_fc);
  let contained_point_features: Vec<_> = points_fc
    .features
    .into_iter()
    .filter(|point_feature| match &point_feature.geometry {
      None => false,
      Some(geojson_geometry) => match &geojson_geometry.value {
        geojson::Value::Point(point_type) => {
          let point_geometry = create_geo_point::<f64>(&point_type);
          let point_is_contained = polygon_geometries.iter().any(|geometry_with_bounds| {
            //if !geometry_with_bounds.bounding_rect.contains(&point_geometry) {
            // early out if point is not within the bounding rect
            //  false
            //} else {
            match &geometry_with_bounds.geometry {
              Geometry::Polygon(polygon) => polygon.contains(&point_geometry),
              Geometry::MultiPolygon(multi_polygon) => multi_polygon.contains(&point_geometry),
              _ => false,
            }
            //}
          });
          point_is_contained
        }
        _ => false,
      },
    })
    .collect();

  if contained_point_features.is_empty() {
    None
  } else {
    let result = geojson::FeatureCollection {
      bbox: None,
      features: contained_point_features,
      foreign_members: None,
    };
    Some(result)
  }
}
