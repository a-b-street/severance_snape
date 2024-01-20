<script lang="ts">
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import type { MapMouseEvent } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import {
    GeoJSON,
    LineLayer,
    Popup,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { colorScale, kindToColor, limits } from "./colors";
  import {
    constructMatchExpression,
    makeColorRamp,
    notNull,
    SequentialLegend,
  } from "./common";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, model } from "./stores";

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
        })
      );
    } catch (err) {
      window.alert(`No route: ${err}`);
      route_gj = null;
    }
  }

  onMount(() => {
    $map?.on("click", onClick);
  });
  onDestroy(() => {
    $map?.off("click", onClick);
  });
  function onClick(e: MapMouseEvent) {
    // If we click off a severance line, clear things
    if (
      $map!.queryRenderedFeatures(e.point, {
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
    <SequentialLegend {colorScale} {limits} />
  </div>
  <div slot="map">
    <GeoJSON data={JSON.parse(notNull($model).render())}>
      <LineLayer
        id="network"
        paint={{
          "line-width": 5,
          "line-color": constructMatchExpression(
            ["get", "kind"],
            kindToColor,
            "yellow"
          ),
          "line-opacity": showSeverances
            ? opacity / 100.0
            : constructMatchExpression(
                ["get", "kind"],
                {
                  Severance: 0.0,
                },
                opacity / 100.0
              ),
        }}
      />
    </GeoJSON>
    <GeoJSON data={JSON.parse(notNull($model).makeHeatmap())}>
      <LineLayer
        id="scores"
        paint={{
          "line-width": 8,
          "line-color": makeColorRamp(["get", "score"], limits, colorScale),
        }}
        on:click={showRoute}
      >
        <Popup openOn="hover" let:data>
          <span style="font-size: 26px"
            >{notNull(data).properties.score.toFixed(1)}x</span
          >
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
