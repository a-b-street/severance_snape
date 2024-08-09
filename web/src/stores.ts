import { MapModel } from "backend";
import type { FeatureCollection } from "geojson";
import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "title" }
  | { kind: "score" }
  | { kind: "route"; route_a: [number, number]; route_b: [number, number] }
  | { kind: "debug" };
export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  route_length: number;
}

export let mode: Writable<Mode> = writable({ kind: "title" });
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
export let showAbout: Writable<boolean> = writable(true);
export let profile = writable("USA");

export let minScore = writable(0);
export let maxScore = writable(100);

export function routeMode(): Mode {
  let bbox: number[] = Array.from(get(model)!.getBounds());
  return {
    kind: "route",
    route_a: [lerp(0.4, bbox[0], bbox[2]), lerp(0.4, bbox[1], bbox[3])],
    route_b: [lerp(0.6, bbox[0], bbox[2]), lerp(0.6, bbox[1], bbox[3])],
  };
}

function lerp(pct: number, a: number, b: number): number {
  return a + pct * (b - a);
}
