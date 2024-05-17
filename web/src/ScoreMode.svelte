<script lang="ts">
  import { SequentialLegend, notNull } from "svelte-utils";
  import { Popup, makeColorRamp } from "svelte-utils/map";
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import type { MapMouseEvent } from "maplibre-gl";
  import {
    MapEvents,
    GeoJSON,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { colorScale, limits } from "./colors";
  import NetworkLayer from "./NetworkLayer.svelte";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { map, mode, model } from "./stores";

  export let showSeverances: boolean;
  export let opacity: number;

  let route_gj: FeatureCollection | null = null;

  // TODO hack... need to toggle off interactiveness of network layer, so just copy it?

  function showRoute(e: CustomEvent<LayerClickInfo>) {
    try {
      let linestring = (e.detail.features[0] as Feature<LineString>).geometry
        .coordinates;
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: linestring[0][0],
          y1: linestring[0][1],
          x2: linestring[1][0],
          y2: linestring[1][1],
        }),
      );
    } catch (err) {
      window.alert(`No route: ${err}`);
      route_gj = null;
    }
  }

  function onClick(e: CustomEvent<MapMouseEvent>) {
    // If we click off a severance line, clear things
    if (
      $map!.queryRenderedFeatures(e.detail.point, {
        layers: ["scores"],
      }).length > 0
    ) {
      return;
    }
    route_gj = null;
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Score mode</h2>
    <div>
      <button on:click={() => ($mode = "title")}>Change study area</button>
      <button on:click={() => ($mode = "route")}>Route mode</button>
    </div>
    <button on:click={() => ($mode = "debug")}>Debug OSM</button>
    <p>
      The desire lines are coloured based on their detour factor. <b>Click</b> one
      to see the route
    </p>
    <SequentialLegend {colorScale} {limits} />
  </div>
  <div slot="map">
    <MapEvents on:click={onClick} />

    <NetworkLayer {showSeverances} {opacity} />

    <GeoJSON data={JSON.parse(notNull($model).makeHeatmap())}>
      <LineLayer
        id="scores"
        paint={{
          "line-width": 8,
          "line-color": makeColorRamp(["get", "score"], limits, colorScale),
        }}
        on:click={showRoute}
      >
        <Popup openOn="hover" let:props>
          <span style="font-size: 26px">{props.score.toFixed(1)}x</span>
        </Popup>
      </LineLayer>
    </GeoJSON>

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
