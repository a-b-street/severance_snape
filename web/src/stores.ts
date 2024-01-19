import { MapModel } from "backend";
import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";

export let sidebarContents: Writable<HTMLDivElement | null> = writable(null);
export let mapContents: Writable<HTMLDivElement | null> = writable(null);

export type Mode = "score" | "route";

export let mode: Writable<Mode> = writable("score");
export let model: Writable<MapModel | null> = writable(null);
export let map: Writable<Map | null> = writable(null);
