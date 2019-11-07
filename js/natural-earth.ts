/**
 * This script illustrates how the fixtures/natural-earth/*_result.geojson files
 * were generated, using turf.js results as the canonical reference.
 */
import { pointsWithinPolygon } from '@turf/turf';
import { readFileSync } from 'fs';

const pointsFeatureCollection = JSON.parse(readFileSync('./fixtures/natural-earth/points-10.geojson').toString());

// simple polygons
const polygons = JSON.parse(readFileSync('./fixtures/natural-earth/ne_110m_land.geojson').toString());

// complex polygons
// const polygons = JSON.parse(readFileSync('./fixtures/natural-earth/ne_10m_land.geojson').toString());

const result = pointsWithinPolygon(pointsFeatureCollection, polygons);
// console.log(JSON.stringify(result, null, 2));
