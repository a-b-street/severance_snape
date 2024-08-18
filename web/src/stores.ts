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

export let mode: Writable<Mode> = writable({ kind: "title" });
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
// TODO Hide if restoring from a URL, or use a local storage bit?
export let showAbout: Writable<boolean> = writable(true);
export let profile = urlState({
  name: "profile",
  defaultValue: "USA",
  stringify: (x) => x,
  parse: enumUrl(["USA", "SidewalksOnHighways", "SeparateWays"]),
});

export let routeA: Writable<Position | null> = writable(null);
export let routeB: Writable<Position | null> = writable(null);
export let minScore = writable(0);
export let maxScore = writable(100);

export type Position = [number, number];
