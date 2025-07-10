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
  | { kind: "isochrone" }
  | { kind: "crossings" }
  | { kind: "debug" }
  | { kind: "disconnected" };

export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  route_length: number;
  active_duration_s: number;
  waiting_duration_s: number;
  directions: Step[];
}

export interface Step {
  name?: string;
  way: string;
  kind: string;
  layer: string;
}

export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
export let offlineMode = writable(false);

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

export let routeA: Writable<Position | null> = writable(null);
export let routeB: Writable<Position | null> = writable(null);
export let minScore: Writable<number> = writable(0);
export let maxScore: Writable<number> = writable(100);
export let isochroneMins: Writable<number> = writable(15);

export type Position = [number, number];

export interface Settings {
  obey_crossings: boolean;
  base_speed_mph: number;
  use_gradient: boolean;
  delay_signalized: number;
  delay_zebra: number;
  delay_other: number;
}

export let settings: Writable<Settings> = writable({
  obey_crossings: true,
  base_speed_mph: 3,
  use_gradient: false,
  delay_signalized: 30,
  delay_zebra: 0,
  delay_other: 10,
});

// Used in isochrone mode sometimes
export let settings2: Writable<Settings> = writable({
  obey_crossings: false,
  base_speed_mph: 3,
  use_gradient: false,
  delay_signalized: 30,
  delay_zebra: 0,
  delay_other: 10,
});
