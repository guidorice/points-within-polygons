import { points_within_polygons as wasmPointsWithinPolygons } from 'points-within-polygons-wasm';
import { readFileSync } from 'fs';

const pointsFeatureCollection = JSON.parse(readFileSync('./fixtures/natural-earth/points-1000.geojson').toString());
const simplePolygonsFeatureCollection = JSON.parse(readFileSync('./fixtures/natural-earth/ne_110m_land.geojson').toString());
const result = wasmPointsWithinPolygons(pointsFeatureCollection, simplePolygonsFeatureCollection);

console.log(JSON.stringify(result, null, 2));
