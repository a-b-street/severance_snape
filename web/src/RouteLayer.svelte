<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import type { LngLat, Map, MapMouseEvent } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import { GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import { map } from "./stores";

  export let route_a: LngLat;
  export let route_b: LngLat;
  export let route_gj: FeatureCollection;

  onMount(() => {
    $map?.on("contextmenu", onRightClick);
  });
  onDestroy(() => {
    $map?.off("contextmenu", onRightClick);
  });
  function onRightClick(e: MapMouseEvent) {
    // Move the first marker, for convenience
    route_a = e.lngLat;
  }
</script>

{#if route_a}
  <Marker bind:lngLat={route_a} draggable><span class="dot">A</span></Marker>
  <Marker bind:lngLat={route_b} draggable><span class="dot">B</span></Marker>
{/if}
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

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: inline-block;
    background-color: grey;
    text-align: center;
  }
</style>
