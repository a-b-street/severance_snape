<script lang="ts">
  import { SequentialLegend } from "svelte-utils";
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
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { map, model, mode, minScore, maxScore } from "./stores";
  import NavBar from "./NavBar.svelte";

  // TODO Cache
  let scoreGj: FeatureCollection<LineString, { score: number }> = JSON.parse(
    $model!.makeHeatmap(),
  );
  let highestScore = Math.round(
    Math.max(...scoreGj.features.map((f) => f.properties.score)),
  );
  if ($maxScore > highestScore) {
    $minScore = 0;
    $maxScore = highestScore;
  }

  let desire_line: Feature<LineString, { score: number }> | null = null;
  let route_gj: FeatureCollection | null = null;

  $: if ($minScore >= $maxScore) {
    $minScore = Math.max(0, $maxScore - 1);
    $maxScore = Math.min(highestScore, $minScore + 1);
  }

  // TODO hack... need to toggle off interactiveness of network layer, so just copy it?

  function showRoute(e: CustomEvent<LayerClickInfo>) {
    try {
      desire_line = e.detail.features[0] as Feature<
        LineString,
        { score: number }
      >;
      let linestring = desire_line.geometry.coordinates as [number, number][];
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
      desire_line = null;
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
    desire_line = null;
    route_gj = null;
  }

  function gotoRouteMode() {
    if (desire_line) {
      $mode = {
        kind: "route",
        route_a: desire_line.geometry.coordinates[0] as [number, number],
        route_b: desire_line.geometry.coordinates[1] as [number, number],
      };
    }
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Score mode</h2>
    <p>
      The desire lines are coloured based on their detour factor. <b>Click</b> one
      to see the route
    </p>
    <SequentialLegend {colorScale} {limits} />

    <fieldset>
      <label
        >Show desire lines with scores {$minScore}-{$maxScore}:
        <input type="range" bind:value={$minScore} min="0" max={highestScore} />
        to
        <input type="range" bind:value={$maxScore} min="0" max={highestScore} />
      </label>
    </fieldset>

    <hr />

    <button on:click={gotoRouteMode} disabled={desire_line == null}
      >See this route in detail</button
    >
    {#if desire_line}
      <p>
        Detour factor: <b>{desire_line.properties.score.toFixed(1)}x</b>
        longer than straight line
      </p>
    {/if}
  </div>
  <div slot="map">
    <MapEvents on:click={onClick} />

    <GeoJSON data={scoreGj}>
      <LineLayer
        id="scores"
        filter={[
          "all",
          [">=", ["get", "score"], $minScore],
          ["<=", ["get", "score"], $maxScore],
        ]}
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
