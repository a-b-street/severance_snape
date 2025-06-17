<script lang="ts">
  import {
    GeoJSON,
    CircleLayer,
    LineLayer,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { notNull, SequentialLegend } from "svelte-utils";
  import { Popup, makeRamp } from "svelte-utils/map";
  import { model } from "./stores";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import NavBar from "./NavBar.svelte";
  import { colorScale } from "./colors";

  let limits = [1, 100, 200, 400, 800, 5000];
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Crossings mode</h2>
    <p>
      This is only showing crossings over severances. For other streets, it's
      assumed that it's easy enough to cross the street anywhere, even without
      explicit crossings.
    </p>

    <SequentialLegend {colorScale} labels={{ limits }} />
  </div>
  <div slot="map">
    <GeoJSON
      data={JSON.parse(notNull($model).getCrossingDistances())}
      generateId
    >
      <LineLayer
        beforeId="Road labels"
        paint={{
          "line-width": hoverStateFilter(5, 10),
          "line-color": makeRamp(["get", "length"], limits, colorScale),
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          {Math.round(props.length)}m
        </Popup>
      </LineLayer>
    </GeoJSON>

    <GeoJSON data={JSON.parse(notNull($model).getCrossings())}>
      <CircleLayer
        paint={{
          "circle-radius": ["step", ["zoom"], 0, 12, 3, 14, 5, 15, 7],
          "circle-color": "yellow",
          "circle-stroke-color": "black",
          "circle-stroke-width": 1,
        }}
      />
    </GeoJSON>
  </div>
</SplitComponent>
