<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { PropertiesTable, notNull } from "svelte-utils";
  import { Popup, constructMatchExpression } from "svelte-utils/map";
  import { model } from "./stores";
  import NavBar from "./NavBar.svelte";

  export let showSeverances: boolean;
  export let opacity: number;
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Debug mode</h2>
    <p>Hover to see a segment's properties, and click to open OSM</p>
  </div>
  <div slot="map">
    <GeoJSON data={JSON.parse(notNull($model).render())} generateId>
      <LineLayer
        id="network"
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": constructMatchExpression(
            ["get", "kind"],
            kindToColor,
            "yellow",
          ),
          "line-opacity": showSeverances
            ? opacity / 100
            : constructMatchExpression(
                ["get", "kind"],
                {
                  Severance: 0.0,
                },
                opacity / 100.0,
              ),
        }}
        manageHoverState
        on:click={(e) =>
          window.open(notNull(e.detail.features[0].properties).way, "_blank")}
        hoverCursor="pointer"
      >
        <Popup openOn="hover" let:props>
          <h2>Classified as {props.kind}</h2>
          <PropertiesTable properties={props} />
        </Popup>
      </LineLayer>
    </GeoJSON>
  </div>
</SplitComponent>
