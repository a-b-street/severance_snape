<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import logo from "../assets/logo.svg?url";
  import init, { MapModel } from "backend";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { Legend } from "svelte-utils";
  import { Geocoder } from "svelte-utils/map";
  import DebugMode from "./DebugMode.svelte";
  import RouteMode from "./RouteMode.svelte";
  import ScoreMode from "./ScoreMode.svelte";
  import CrossingsMode from "./CrossingsMode.svelte";
  import OsmSeparateSidewalksMode from "./OsmSeparateSidewalksMode.svelte";
  import NetworkLayer from "./NetworkLayer.svelte";
  import {
    map as mapStore,
    mode,
    model,
    maptilerApiKey,
    showAbout,
    type Mode,
  } from "./stores";
  import TitleMode from "./title/TitleMode.svelte";
  import DisconnectionsMode from "./DisconnectionsMode.svelte";
  import {
    Layout,
    mapContents,
    sidebarContents,
    topContents,
  } from "svelte-utils/top_bar_layout";
  import About from "./About.svelte";
  // TODO Indirect dependencies
  import * as pmtiles from "pmtiles";
  import maplibregl from "maplibre-gl";

  let offlineMode = false;
  if (offlineMode) {
    let protocol = new pmtiles.Protocol();
    maplibregl.addProtocol("pmtiles", protocol.tile);
  }

  let wasmReady = false;
  onMount(async () => {
    await init();
    wasmReady = true;
  });

  let fitBoundsAtStart = !window.location.hash;
  let restoreMode = parseMode();

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  let opacity = 100;
  let showSeverances = true;

  // We always have to go through TitleMode to load the study area, so we have to restore the Mode a little carefully
  function parseMode(): Mode {
    let value = new URLSearchParams(window.location.search).get("mode") || "";
    // Exclude title from this list; don't stay here
    if (
      [
        "score",
        "route",
        "crossings",
        "debug",
        "disconnected",
        "osm-separate-sidewalks",
      ].includes(value)
    ) {
      return { kind: value } as Mode;
    }
    return { kind: "route" };
  }

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
    console.log(`New map model loaded. Starting in ${restoreMode.kind}`);
    if (fitBoundsAtStart) {
      zoomToFit();
    }
    fitBoundsAtStart = true;
    $mode = restoreMode;
    restoreMode = { kind: "route" };
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
      <img src={logo} style="height: 6vh;" alt="A/B Street logo" />
    </button>
    <span bind:this={topDiv} style="margin-left: 4px; width: 100%" />
  </div>
  <div slot="left">
    <h1>Severance Snape</h1>
    <div bind:this={sidebarDiv} />

    {#if $mode.kind != "title" && $mode.kind != "crossings"}
      <hr />
      <div><button on:click={zoomToFit}>Zoom to fit</button></div>

      {#if $mode.kind != "osm-separate-sidewalks"}
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
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={offlineMode
        ? "http://localhost:5173/offline/light_style.json"
        : `https://api.maptiler.com/maps/landscape/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      {#if !offlineMode}
        <Geocoder {map} apiKey={maptilerApiKey} country={undefined} />
      {/if}
      <div bind:this={mapDiv} />

      {#if $mode.kind == "title"}
        <TitleMode {wasmReady} />
      {/if}
      {#if $model}
        <GeoJSON data={JSON.parse($model.getInvertedBoundary())}>
          <FillLayer paint={{ "fill-color": "black", "fill-opacity": 0.3 }} />
        </GeoJSON>

        <NetworkLayer
          {offlineMode}
          show={$mode.kind != "debug" &&
            $mode.kind != "osm-separate-sidewalks" &&
            $mode.kind != "disconnected" &&
            $mode.kind != "crossings"}
          {showSeverances}
          {opacity}
        />

        {#if $mode.kind == "route"}
          <RouteMode />
        {:else if $mode.kind == "score"}
          <ScoreMode />
        {:else if $mode.kind == "crossings"}
          <CrossingsMode />
        {:else if $mode.kind == "debug"}
          <DebugMode {showSeverances} {opacity} />
        {:else if $mode.kind == "osm-separate-sidewalks"}
          <OsmSeparateSidewalksMode />
        {:else if $mode.kind == "disconnected"}
          <DisconnectionsMode />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>

<style>
  :global(.maplibregl-popup-content) {
    background-color: var(--pico-background-color);
  }

  /* picocss messes up maplibre controls; workaround */
  :global(.maplibregl-ctrl > button) {
    margin-bottom: 0px;
  }
</style>
