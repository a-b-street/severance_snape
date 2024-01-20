<script lang="ts">
  import init, { MapModel } from "backend";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { Layout, Legend } from "./common";
  import PolygonToolLayer from "./common/draw_polygon/PolygonToolLayer.svelte";
  import DebugMode from "./DebugMode.svelte";
  import RouteMode from "./RouteMode.svelte";
  import ScoreMode from "./ScoreMode.svelte";
  import {
    mapContents,
    map as mapStore,
    mode,
    model,
    sidebarContents,
  } from "./stores";
  import TitleMode from "./title/TitleMode.svelte";

  let wasmReady = false;
  onMount(async () => {
    await init();
    wasmReady = true;
  });

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  let opacity = 100;
  let showSeverances = true;

  function zoomToFit() {
    if (map && $model) {
      map.fitBounds(
        Array.from($model.getBounds()) as [number, number, number, number],
        { animate: false }
      );
    }
  }

  function gotModel(_m: MapModel | null) {
    if (!$model) {
      return;
    }
    console.log("New map model loaded");
    zoomToFit();
    $mode = "route";
  }
  $: gotModel($model);

  let sidebarDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<Layout>
  <div slot="left">
    <div bind:this={sidebarDiv} />

    {#if $mode != "title"}
      <hr />
      <div><button on:click={zoomToFit}>Zoom to fit</button></div>

      <Legend
        rows={[
          ["Footway (ground, outdoors)", kindToColor.Footway],
          ["Indoors footway", kindToColor.Indoors],
          ["Footway not on the ground", kindToColor.BridgeOrTunnel],
          ["Street with sidewalk (or pedestrian street)", kindToColor.Sidewalk],
          ["Crossing", kindToColor.Crossing],
          ["Severance", kindToColor.Severance],
        ]}
      />
      <div>
        <label>
          <input type="checkbox" bind:checked={showSeverances} />
          Show severances
        </label>
      </div>
      <div>
        <label>
          Network opacity:
          <input type="range" min="0" max="100" bind:value={opacity} />
        </label>
      </div>
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      <div bind:this={mapDiv} />

      <PolygonToolLayer />
      {#if $mode == "title"}
        <TitleMode {wasmReady} />
      {/if}
      {#if $model}
        <GeoJSON data={JSON.parse($model.getInvertedBoundary())}>
          <FillLayer paint={{ "fill-color": "black", "fill-opacity": 0.3 }} />
        </GeoJSON>
        {#if $mode == "route"}
          <RouteMode {showSeverances} {opacity} />
        {:else if $mode == "score"}
          <ScoreMode {showSeverances} {opacity} />
        {:else if $mode == "debug"}
          <DebugMode {showSeverances} {opacity} />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>

<style>
  :global(body, button, input) {
    font-size: 26px;
  }
</style>
