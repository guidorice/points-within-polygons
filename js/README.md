# Javascript benchmarks for points-within-polygons project

`benchmark.ts` measures the performance of Turf.js pointsWithinPolygon function
 http://turfjs.org/docs/#pointsWithinPolygon

```bash
# install js and ts dependencies
$ yarn
yarn install v1.19.1
[1/4] Resolving packages...
[2/4] Fetching packages...
[3/4] Linking dependencies...
[4/4] Building fresh packages...
Done in 3.48s.
# run the benchmark script
$ yarn run benchmark
yarn run v1.19.1
$ tsc benchmark.ts && node benchmark.js
pointsWithinPolygon on simple polygons x 12.68 ops/sec ±2.13% (35 runs sampled)
pointsWithinPolygon on complex polygons x 1.10 ops/sec ±1.04% (7 runs sampled)
Fastest is pointsWithinPolygon on simple polygons
Done in 23.32s.
```
