<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import { MapEvents, GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import Directions from "./Directions.svelte";
  import NetworkLayer from "./NetworkLayer.svelte";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { mode, model, type RouteGJ } from "./stores";

  export let showSeverances: boolean;
  export let opacity: number;

  // TODO Maybe need to do this when model changes
  let bbox: number[] = Array.from($model!.getBounds());
  let route_a = {
    lng: lerp(0.4, bbox[0], bbox[2]),
    lat: lerp(0.4, bbox[1], bbox[3]),
  };
  let route_b = {
    lng: lerp(0.6, bbox[0], bbox[2]),
    lat: lerp(0.6, bbox[1], bbox[3]),
  };

  // TODO or empty
  let route_gj: RouteGJ | null = null;
  let route_err = "";

  $: if (route_a && route_b) {
    try {
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: route_a.lng,
          y1: route_a.lat,
          x2: route_b.lng,
          y2: route_b.lat,
        }),
      );
      route_err = "";
    } catch (err: any) {
      route_gj = null;
      route_err = err.toString();
    }
  }

  function onRightClick(e: CustomEvent<MapMouseEvent>) {
    // Move the first marker, for convenience
    route_a = e.detail.lngLat;
  }

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Route mode</h2>
    <div>
      <button on:click={() => ($mode = "title")}>Change study area</button>
      <button on:click={() => ($mode = "score")}>Score mode</button>
    </div>
    <button on:click={() => ($mode = "debug")}>Debug OSM</button>
    <p>
      Move the <b>A</b> and <b>B</b> pins to find a walking route. (Hint: right-click
      to set the first pin somewhere.)
    </p>
    {#if route_err}
      <p>{route_err}</p>
    {/if}
    {#if route_gj}
      <Directions {route_gj} />
    {/if}
  </div>
  <div slot="map">
    <MapEvents on:contextmenu={onRightClick} />

    <NetworkLayer {showSeverances} {opacity} />

    <Marker bind:lngLat={route_a} draggable><span class="dot">A</span></Marker>
    <Marker bind:lngLat={route_b} draggable><span class="dot">B</span></Marker>
    {#if route_gj}
      <GeoJSON data={route_gj}>
        <LineLayer
          id="route"
          beforeId="network"
          paint={{
            "line-width": 20,
            "line-color": "cyan",
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    color: white;
    background-color: blue;
    font-weight: bold;
  }
</style>
