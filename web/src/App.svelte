<script lang="ts">
  import turfBbox from "@turf/bbox";
  import { MapModel } from "backend";
  import type { FeatureCollection } from "geojson";
  import type { LngLat, Map } from "maplibre-gl";
  import { MapLibre } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { Layout, Legend } from "./common";
  import PolygonToolLayer from "./common/draw_polygon/PolygonToolLayer.svelte";
  import MapLoader from "./MapLoader.svelte";
  import RouteMode from "./RouteMode.svelte";
  import ScoreMode from "./ScoreMode.svelte";
  import {
    mapContents,
    map as mapStore,
    mode,
    model,
    sidebarContents,
  } from "./stores";

  // TODO do the wasm stuff here

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  let opacity = 100;
  let showSeverances = true;

  function zoomToFit() {
    if (map && $model) {
      // TODO wasteful
      let bbox = turfBbox(JSON.parse($model.render()));
      map.fitBounds(bbox, { animate: false });
    }
  }

  function gotModel(_m: MapModel) {
    if (!$model) {
      return;
    }
    console.log("New map model loaded");
    zoomToFit();
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

    {#if map}
      <MapLoader />
    {/if}
    <div><button on:click={zoomToFit}>Zoom to fit</button></div>

    <label>
      <input bind:group={$mode} type="radio" value="route" />Route
    </label>
    <label>
      <input bind:group={$mode} type="radio" value="score" />Score
    </label>

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
      {#if $model}
        {#if $mode == "route"}
          <RouteMode {showSeverances} {opacity} />
        {:else if $mode == "score"}
          <ScoreMode {showSeverances} {opacity} />
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
