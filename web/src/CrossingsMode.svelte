<script lang="ts">
  import {
    GeoJSON,
    CircleLayer,
    LineLayer,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { model } from "./stores";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import NavBar from "./NavBar.svelte";
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
  </div>
  <div slot="map">
    <GeoJSON data={JSON.parse(notNull($model).render())} generateId>
      <LineLayer
        filter={["==", ["get", "kind"], "Severance"]}
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": "red",
        }}
        manageHoverState
      />
    </GeoJSON>

    <GeoJSON data={JSON.parse(notNull($model).getCrossings())}>
      <CircleLayer
        paint={{
          "circle-radius": 7,
          "circle-color": "yellow",
          "circle-stroke-color": "black",
          "circle-stroke-width": 1,
        }}
      />
    </GeoJSON>
  </div>
</SplitComponent>
