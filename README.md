# points-within-polygons

A points in polygons spatial join for Rust and WebAssembly.

## Background

🌵Coming soon: blog post about this, written as someone completely new to Rust.

I modeled this after the [Turf.js](http://turfjs.org/)
[pointsWithinPolygon](http://turfjs.org/docs/#pointsWithinPolygon) function and
implemented (approximately) the same function signature in Rust. I tried to
implement the same algorithm as well, to make a fair comparison of the
[GeoRust](https://github.com/georust) crates which are utilized by this crate.
The Rust function signature therefore is:

```rust
pub fn points_within_polygons(
  points_fc: geojson::FeatureCollection,
  polygons_fc: geojson::FeatureCollection,
) -> Option<geojson::FeatureCollection>
```

## Requirements

- [Node.js](https://nodejs.org/en/)
- [Yarn](https://yarnpkg.com/lang/en/)
- Rust and wasm-pack: https://rustwasm.github.io/wasm-pack/book/quickstart.html
- Binaryen's wasm-opt: https://github.com/WebAssembly/binaryen

## To build

```bash
git clone https://github.com/guidorice/points-within-polygons.git
cd points-within-polygons
cargo build
```

## Run Rust unit and integration tests

```bash
cargo test
```

## Run Rust native benchmark tests

Note: the native benchmarks also unfortunately include `.clone()` calls, so are
not really measuring the performance of the function.

```bash
cargo +nightly bench
```

## Run Node.js benchmarks to compare Turf.js vs this Rust crate as WebAssembly

```bash
cd wasm
rm -rf pkg
wasm-pack build --target nodejs
# See below about optimizing the .wasm file. Do this before running yarn in js/.
cd ../js
rm -rf node_modules
yarn # installs packages into node_modules/
yarn run benchmark # runs benchmark script from package.json
```

## Recommended: run binaryen's wasm-opt optimizer

TODO: This may now be an option in wasm-pack's Cargo.toml? If so, get rid of
manual steps

```bash
cd wasm/pkg
wasm-opt -O4 -o optimized.wasm points_within_polygons_wasm_bg.wasm
mv points_within_polygons_wasm_bg.wasm original.wasm
mv optimized.wasm points_within_polygons_wasm_bg.wasm
cd ../..
# now continue with js/ directory above
```

## Example benchmark

```bash
$ yarn run benchmark
yarn run v1.19.1
$ node_modules/typescript/bin/tsc benchmark.ts && node benchmark.js
turf.js/pointsWithinPolygon on simple polygons x 10 points x 69.87 ops/sec ±15.31% (52 runs sampled)
rust-wasm pointsWithinPolgons on simple polygons x 10 points x 81.44 ops/sec ±12.67% (51 runs sampled)
turf.js/pointsWithinPolygon on simple polygons x 100 points x 8.88 ops/sec ±15.98% (25 runs sampled)
rust-wasm pointsWithinPolgons on simple polygons x 100 points x 62.52 ops/sec ±8.61% (64 runs sampled)
turf.js/pointsWithinPolygon on simple polygons x 1000 points x 1.16 ops/sec ±8.90% (7 runs sampled)
rust-wasm pointsWithinPolgons on simple polygons x 1000 points x 12.72 ops/sec ±8.40% (35 runs sampled)
turf.js/pointsWithinPolygon on complex polygons x 10 points x 10.68 ops/sec ±5.37% (31 runs sampled)
rust-wasm pointsWithinPolgons on complex polygons x 10 points x 1.82 ops/sec ±6.67% (9 runs sampled)
turf.js/pointsWithinPolygon on complex polygons x 100 points x 1.07 ops/sec ±7.52% (7 runs sampled)
rust-wasm pointsWithinPolgons on complex polygons x 100 points x 1.18 ops/sec ±5.33% (8 runs sampled)
turf.js/pointsWithinPolygon on complex polygons x 1000 points x 0.12 ops/sec ±12.22% (5 runs sampled)
rust-wasm pointsWithinPolgons on complex polygons x 1000 points x 0.24 ops/sec ±11.19% (5 runs sampled)
✨  Done in 216.71s.
```