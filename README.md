# Severance Snape

This is an **experimental** tool to study "severances" for people walking. In
some places, crossing a big road (a "severance") might be easy -- there's a
zebra or signalized crossing right on the "desire line" where someone might
want to cross. But often, a person might have to walk a few blocks to reach the
nearest crossing -- and then they might have to go up or down stairs to cross
on a bridge or a tunnel!

The app itself explains more: <https://dabreegster.github.io/severance_snape>

## Developer guide

If you want to run this locally, you'll need [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

```
cd web
# Once, to install web dependencies
npm i
# Every time you modify the Rust code
npm run wasm
# To launch the local server
npm run dev
```

`backend` contains a Rust crate that gets compiled to WASM and runs in the browser. The frontend passes it OSM XML or PBF, and the Rust code builds a routable walking network in `scrape.rs`. `lib.rs` defines the network structure and also the WASM API. The API generally returns GeoJSON as a string.

`web` is the frontend, using Svelte and MapLibre via (shockingly) [svelte-maplibre](https://github.com/dimfeld/svelte-maplibre/). Global variables are in `stores.ts`. The app is split into distinct modes.
