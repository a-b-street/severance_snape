<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import { MapEvents, GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import Directions from "./Directions.svelte";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { model, type RouteGJ } from "./stores";
  import NavBar from "./NavBar.svelte";

  export let route_a: [number, number];
  export let route_b: [number, number];

  // TODO or empty
  let route_gj: RouteGJ | null = null;
  let route_err = "";

  $: if (route_a && route_b) {
    try {
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: route_a[0],
          y1: route_a[1],
          x2: route_b[0],
          y2: route_b[1],
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
    route_a = e.detail.lngLat.toArray();
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Route mode</h2>
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
