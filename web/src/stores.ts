import { MapModel } from "backend";
import type { FeatureCollection } from "geojson";
import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode = "title" | "score" | "route" | "debug";
export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  route_length: number;
}

export let mode: Writable<Mode> = writable("title");
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
export let showAbout: Writable<boolean> = writable(true);
export let importStreetsWithoutSidewalkTagging: Writable<boolean> =
  writable(true);
