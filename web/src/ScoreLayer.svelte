<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { GeoJSON, LineLayer, Popup } from "svelte-maplibre";
  import { colorScale, limits } from "./colors";
  import { constructMatchExpression, makeColorRamp } from "./common";
  import RouteLayer from "./RouteLayer.svelte";

  export let model;
  export let map;
  export let showSeverances: boolean;
  export let opacity: number;

  let route_gj = null;

  // TODO hack... need to toggle off interactiveness of network layer, so just copy it?

  function showRoute(e) {
    try {
      let linestring = e.detail.features[0].geometry.coordinates;
      route_gj = JSON.parse(
        model.compareRoute({
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
    map?.on("click", onClick);
  });
  onDestroy(() => {
    map?.off("click", onClick);
  });
  function onClick(e) {
    // If we click off a severance line, clear things
    for (let f of map.queryRenderedFeatures(e.point, {
      layers: ["scores"],
    })) {
      return;
    }
    route_gj = null;
  }
</script>

<GeoJSON data={JSON.parse(model.render())}>
  <LineLayer
    id="network"
    paint={{
      "line-width": 5,
      "line-color": constructMatchExpression(
        ["get", "kind"],
        {
          Footway: "red",
          Indoors: "blue",
          BridgeOrTunnel: "purple",
          Sidewalk: "black",
          Crossing: "green",
          Severance: "orange",
        },
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
<GeoJSON data={JSON.parse(model.makeHeatmap())}>
  <LineLayer
    id="scores"
    paint={{
      "line-width": 8,
      "line-color": makeColorRamp(["get", "score"], limits, colorScale),
    }}
    on:click={showRoute}
  >
    <Popup openOn="hover" let:data
      >{@html JSON.stringify(data.properties, null, "<br />")}</Popup
    >
  </LineLayer>
</GeoJSON>
<RouteLayer {route_gj} route_a={null} route_b={null} {map} />
