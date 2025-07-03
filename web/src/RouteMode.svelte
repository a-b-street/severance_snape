<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import { MapEvents, GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import Directions from "./Directions.svelte";
  import ChangeSettings from "./ChangeSettings.svelte";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { model, type RouteGJ, routeA, routeB, settings } from "./stores";
  import NavBar from "./NavBar.svelte";
  import { onMount } from "svelte";

  // TODO or empty
  let route_gj: RouteGJ | null = null;
  let route_err = "";

  onMount(() => {
    if ($routeA) {
      return;
    }

    let bbox: number[] = Array.from($model!.getBounds());
    $routeA = [lerp(0.4, bbox[0], bbox[2]), lerp(0.4, bbox[1], bbox[3])];
    $routeB = [lerp(0.6, bbox[0], bbox[2]), lerp(0.6, bbox[1], bbox[3])];
  });

  $: if ($routeA && $routeB) {
    try {
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: $routeA[0],
          y1: $routeA[1],
          x2: $routeB[0],
          y2: $routeB[1],
          settings: $settings,
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
    $routeA = e.detail.lngLat.toArray();
  }

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Route mode</h2>
    <p>
      Move the <b>A</b> and <b>B</b> pins to find a route. (Hint: right-click to
      set the first pin somewhere.)
    </p>
    {#if route_err}
      <p>{route_err}</p>
    {/if}
    {#if route_gj}
      <Directions {route_gj} />
    {/if}
    <ChangeSettings open />
  </div>
  <div slot="map">
    <MapEvents on:contextmenu={onRightClick} />

    {#if $routeA && $routeB}
      <Marker bind:lngLat={$routeA} draggable><span class="dot">A</span></Marker
      >
      <Marker bind:lngLat={$routeB} draggable><span class="dot">B</span></Marker
      >
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
