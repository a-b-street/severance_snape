import { MapModel } from "backend";
import type { FeatureCollection } from "geojson";
import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";
import { urlState, enumUrl } from "./url";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "title" }
  | { kind: "score" }
  | { kind: "route" }
  | { kind: "debug" }
  | { kind: "disconnected" }
  | { kind: "osm-separate-sidewalks" };

export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  route_length: number;
}

export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);

// Don't use urlState, because we have to manually go through title mode first
export let mode: Writable<Mode> = writable({ kind: "title" });
// TODO How do we avoid leaking this?
mode.subscribe((state) => {
  // There's a race condition with App doing parseMode. Don't set title mode.
  if (state.kind == "title") {
    return;
  }

  let url = new URL(window.location.href);
  url.searchParams.set("mode", state.kind);
  window.history.replaceState(null, "", url.toString());
});

// TODO Hide if restoring from a URL, or use a local storage bit?
export let showAbout: Writable<boolean> = writable(true);
export let profile = urlState({
  name: "profile",
  defaultValue: "USA",
  stringify: (x) => x,
  parse: enumUrl(["USA", "SidewalksOnHighways", "SeparateWays"]),
});

export let routeA: Writable<Position | null> = urlState({
  name: "routeA",
  defaultValue: null,
  stringify: (pt) => (pt ? `${pt[0]},${pt[1]}` : null),
  parse: parsePt,
});
export let routeB: Writable<Position | null> = urlState({
  name: "routeB",
  defaultValue: null,
  stringify: (pt) => (pt ? `${pt[0]},${pt[1]}` : null),
  parse: parsePt,
});
export let minScore: Writable<number> = urlState({
  name: "minScore",
  defaultValue: 0,
  stringify: (x) => (x == 0 ? null : x.toString()),
  parse: parsePercent,
});
export let maxScore: Writable<number> = urlState({
  name: "maxScore",
  defaultValue: 100,
  stringify: (x) => (x == 100 ? null : x.toString()),
  parse: parsePercent,
});
export let duplicateSidewalks: Writable<boolean> = urlState({
  name: "duplicateSidewalks",
  defaultValue: true,
  stringify: (x) => (x ? "1" : "0"),
  parse: (x) => x == "1",
});

export type Position = [number, number];

function parsePt(pt: string): Position {
  let [x, y] = pt.split(",");
  return [notNan(parseFloat(x)), notNan(parseFloat(y))];
}

function notNan(n: number): number {
  if (isNaN(n)) {
    throw new Error("not a number");
  }
  return n;
}

function parsePercent(x: string): number {
  let n = notNan(parseInt(x));
  if (n < 0 || n > 100) {
    throw new Error(`bad percent ${n}`);
  }
  return n;
}
