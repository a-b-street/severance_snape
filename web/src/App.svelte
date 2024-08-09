<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import logoDark from "../assets/logo_dark.svg?url";
  import init, { MapModel } from "backend";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { Legend } from "svelte-utils";
  import { Geocoder } from "svelte-utils/map";
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import DebugMode from "./DebugMode.svelte";
  import RouteMode from "./RouteMode.svelte";
  import ScoreMode from "./ScoreMode.svelte";
  import NetworkLayer from "./NetworkLayer.svelte";
  import {
    map as mapStore,
    mode,
    model,
    maptilerApiKey,
    showAbout,
    routeMode,
  } from "./stores";
  import TitleMode from "./title/TitleMode.svelte";
  import {
    Layout,
    mapContents,
    sidebarContents,
    topContents,
  } from "svelte-utils/top_bar_layout";
  import About from "./About.svelte";

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
        { animate: false },
      );
    }
  }

  function gotModel(_m: MapModel | null) {
    if (!$model) {
      return;
    }
    console.log("New map model loaded");
    zoomToFit();
    $mode = routeMode();
  }
  $: gotModel($model);

  let topDiv: HTMLSpanElement;
  let sidebarDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  $: if (topDiv && $topContents) {
    topDiv.innerHTML = "";
    topDiv.appendChild($topContents);
  }
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<About />
<Layout>
  <div slot="top" style="display: flex">
    <button class="outline" on:click={() => ($showAbout = true)}>
      <img src={logoDark} style="height: 6vh;" alt="A/B Street logo" />
    </button>
    <span bind:this={topDiv} style="width: 100%" />
  </div>
  <div slot="left">
    <h1>Severance Snape</h1>
    <div bind:this={sidebarDiv} />

    {#if $mode.kind != "title"}
      <hr />
      <div><button on:click={zoomToFit}>Zoom to fit</button></div>

      <Legend
        rows={[
          ["Footway (ground, outdoors)", kindToColor.Footway],
          ["Indoors footway", kindToColor.Indoors],
          ["Footway not on the ground", kindToColor.BridgeOrTunnel],
          [
            "Street with vehicle traffic (maybe with a sidewalk, maybe not)",
            kindToColor.WithTraffic,
          ],
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
      style={`https://api.maptiler.com/maps/landscape/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      <Geocoder {map} apiKey={maptilerApiKey} />
      <div bind:this={mapDiv} />

      <PolygonToolLayer />
      {#if $mode.kind == "title"}
        <TitleMode {wasmReady} />
      {/if}
      {#if $model}
        <GeoJSON data={JSON.parse($model.getInvertedBoundary())}>
          <FillLayer paint={{ "fill-color": "black", "fill-opacity": 0.3 }} />
        </GeoJSON>

        <NetworkLayer show={$mode.kind != "debug"} {showSeverances} {opacity} />

        {#if $mode.kind == "route"}
          <RouteMode route_a={$mode.route_a} route_b={$mode.route_b} />
        {:else if $mode.kind == "score"}
          <ScoreMode />
        {:else if $mode.kind == "debug"}
          <DebugMode {showSeverances} {opacity} />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>
