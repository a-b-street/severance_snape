<script lang="ts">
  import { FillLayer, GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import ChangeSettings from "./ChangeSettings.svelte";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { SequentialLegend } from "svelte-utils";
  import { isLine, isPolygon, makeRamp, emptyGeojson } from "svelte-utils/map";
  import {
    isochroneMins,
    offlineMode,
    model,
    routeA,
    settings,
  } from "./stores";
  import NavBar from "./NavBar.svelte";
  import { onMount } from "svelte";
  import { colorScale } from "./colors";

  let isochroneGj = emptyGeojson();

  let style = "Roads";
  $: limits = Array.from(Array(6).keys()).map(
    (i) => (($isochroneMins * 60) / (6 - 1)) * i,
  );

  onMount(() => {
    if ($routeA) {
      return;
    }

    let bbox: number[] = Array.from($model!.getBounds());
    $routeA = [lerp(0.4, bbox[0], bbox[2]), lerp(0.4, bbox[1], bbox[3])];
  });

  $: if ($routeA) {
    try {
      isochroneGj = JSON.parse(
        $model!.isochrone({
          x: $routeA[0],
          y: $routeA[1],
          style,
          time_limit: $isochroneMins,
          settings: $settings,
        }),
      );
    } catch (err: any) {
      console.error(err);
      isochroneGj = emptyGeojson();
    }
  }

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Isochrone mode</h2>
    <ChangeSettings />

    <label
      >Draw:
      <select bind:value={style}>
        <option value="Roads">Roads</option>
        <option value="Grid">Grid</option>
        <option value="Contours">Contours</option>
        <option value="Dasymetric">Dasymetric</option>
      </select>
    </label>

    <label
      >Minutes away
      <input type="number" bind:value={$isochroneMins} min="1" max="30" />
    </label>
    <SequentialLegend
      {colorScale}
      labels={{ limits: limits.map((l) => l / 60) }}
    />
  </div>
  <div slot="map">
    {#if $routeA}
      <Marker bind:lngLat={$routeA} draggable><span class="dot">X</span></Marker
      >
    {/if}

    <GeoJSON data={isochroneGj}>
      <LineLayer
        id="isochrone-roads"
        beforeId={$offlineMode ? "roads_labels_major" : "Road labels"}
        filter={isLine}
        paint={{
          "line-width": 2,
          "line-color": makeRamp(["get", "cost_seconds"], limits, colorScale),
        }}
      />

      <FillLayer
        id="isochrone-polygons"
        beforeId={$offlineMode ? "roads_labels_major" : "Road labels"}
        filter={isPolygon}
        paint={{
          "fill-color": makeRamp(["get", "cost_seconds"], limits, colorScale),
          "fill-opacity": style == "Dasymetric" ? 1.0 : 0.5,
        }}
      />
    </GeoJSON>
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
