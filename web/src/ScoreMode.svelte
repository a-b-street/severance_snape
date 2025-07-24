<script lang="ts">
  import { SequentialLegend } from "svelte-utils";
  import { Popup, makeRamp } from "svelte-utils/map";
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
  import {
    map,
    model,
    mode,
    minScore,
    maxScore,
    routeA,
    routeB,
    settings,
    type Position,
  } from "./stores";

  let scoreGj: FeatureCollection<LineString, { score: number }> = JSON.parse(
    $model!.scoreDetours(),
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
      let linestring = desire_line.geometry.coordinates as Position[];
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: linestring[0][0],
          y1: linestring[0][1],
          x2: linestring[1][0],
          y2: linestring[1][1],
          settings: $settings,
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
      $routeA = desire_line.geometry.coordinates[0] as Position;
      $routeB = desire_line.geometry.coordinates[1] as Position;
      $mode = {
        kind: "route",
      };
    }
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Score mode</h2>
    <p>
      The desire lines are coloured based on their detour factor. <b>Click</b> one
      to see the route
    </p>
    <SequentialLegend {colorScale} labels={{ limits }} />

    <label class="form-label"
      >Show desire lines with scores {$minScore}-{$maxScore}:
      <input
        class="form-control"
        type="range"
        bind:value={$minScore}
        min="0"
        max={highestScore}
      />
      to
      <input
        class="form-control"
        type="range"
        bind:value={$maxScore}
        min="0"
        max={highestScore}
      />
    </label>

    <hr />

    <button
      class="btn btn-secondary"
      on:click={gotoRouteMode}
      disabled={desire_line == null}>See this route in detail</button
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
          "line-color": makeRamp(["get", "score"], limits, colorScale),
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
