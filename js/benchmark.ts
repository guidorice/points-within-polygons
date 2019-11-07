import * as benchmark from 'benchmark';
import { pointsWithinPolygon } from '@turf/turf';
import { readFileSync } from 'fs';
import { points_within_polygons as wasmPointsWithinPolygons } from 'points-within-polygons-wasm';

const suite = new benchmark.Suite;
const pointsFeatureCollection_10 = JSON.parse(readFileSync('./fixtures/natural-earth/points-10.geojson').toString());
const pointsFeatureCollection_100 = JSON.parse(readFileSync('./fixtures/natural-earth/points-100.geojson').toString());
const pointsFeatureCollection_1000 = JSON.parse(readFileSync('./fixtures/natural-earth/points-1000.geojson').toString());

const simplePolygonsFeatureCollection = JSON.parse(readFileSync('./fixtures/natural-earth/ne_110m_land.geojson').toString());
const complexPolygonsFeatureCollection = JSON.parse(readFileSync('./fixtures/natural-earth/ne_10m_land.geojson').toString());

const turfMsg = 'turf.js/pointsWithinPolygon on';
const wasmMsg = 'rust-wasm pointsWithinPolgons on';

suite
    .add(`${turfMsg} simple polygons x 10 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_10, simplePolygonsFeatureCollection);
    })
    .add(`${wasmMsg} simple polygons x 10 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_10, simplePolygonsFeatureCollection);
    })
    .add(`${turfMsg} simple polygons x 100 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_100, simplePolygonsFeatureCollection);
    })
    .add(`${wasmMsg} simple polygons x 100 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_100, simplePolygonsFeatureCollection);
    })
    .add(`${turfMsg} simple polygons x 1000 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_1000, simplePolygonsFeatureCollection);
    })
    .add(`${wasmMsg} simple polygons x 1000 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_1000, simplePolygonsFeatureCollection);
    })
    .add(`${turfMsg} complex polygons x 10 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_10, complexPolygonsFeatureCollection);
    })
    .add(`${wasmMsg} complex polygons x 10 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_10, complexPolygonsFeatureCollection);
    })
    .add(`${turfMsg} complex polygons x 100 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_100, complexPolygonsFeatureCollection);
    })
    .add(`${wasmMsg} complex polygons x 100 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_100, complexPolygonsFeatureCollection);
    })
    .add(`${turfMsg} complex polygons x 1000 points`, () => {
        pointsWithinPolygon(pointsFeatureCollection_1000, complexPolygonsFeatureCollection);
    })
    .add(`${wasmMsg} complex polygons x 1000 points`, () => {
        wasmPointsWithinPolygons(pointsFeatureCollection_1000, complexPolygonsFeatureCollection);
    })
    .on('cycle', function (event) {
        console.log(String(event.target));
    })
    .run();
