<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapEvents,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import type { MapMouseEvent, ExpressionSpecification } from "maplibre-gl";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { constructMatchExpression } from "svelte-utils/map";
  import { map, model } from "./stores";
  import NavBar from "./NavBar.svelte";

  let gj = JSON.parse($model!.findConnectedComponents());

  let colors = ["#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e"];
  let colorByComponent = constructMatchExpression(
    ["to-string", ["%", ["get", "component"], colors.length]],
    Object.fromEntries(colors.map((color, i) => [i.toString(), color])),
    "black",
  ) as ExpressionSpecification;

  let showComponent: number | null = null;

  function lineColor(showComponent: number | null): ExpressionSpecification {
    if (showComponent == null) {
      return colorByComponent;
    }
    return [
      "case",
      ["==", ["get", "component"], showComponent],
      colorByComponent,
      "black",
    ];
  }

  function clickLine(e: CustomEvent<LayerClickInfo>) {
    showComponent = e.detail.features[0].properties!.component;
  }

  function onClick(e: CustomEvent<MapMouseEvent>) {
    // If we click off a line, clear things
    if (
      $map!.queryRenderedFeatures(e.detail.point, {
        layers: ["disconnections"],
      }).length == 0
    ) {
      showComponent = null;
    }
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Network disconnections</h2>
    <p>
      This shows where the pedestrian network is disconnected, according to the
      import profile. If there are unexpected cases here, it could either be a
      problem with OSM data, the import profile, or truly reality. Severances
      are <b>not</b> shown.
    </p>

    <p>Component sizes:</p>
    <ul>
      {#each gj.components as size, idx}
        <li style:color={colors[idx % colors.length]}>{size}</li>
      {/each}
    </ul>
  </div>
  <div slot="map">
    <MapEvents on:click={onClick} />
    <GeoJSON data={gj} generateId>
      <LineLayer
        id="disconnections"
        paint={{
          "line-width": hoverStateFilter(5, 10),
          "line-color": lineColor(showComponent),
        }}
        manageHoverState
        hoverCursor="pointer"
        on:click={clickLine}
      />
    </GeoJSON>
  </div>
</SplitComponent>
