<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { PropertiesTable, notNull } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import { model } from "./stores";
  import NavBar from "./NavBar.svelte";
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Fix OSM sidewalk=separate mode</h2>
    <p>
      This shows roads that seem to have a parallel sidewalk. Click one to open
      OSM, then (if appropriate) add <b>sidewalk=separate</b> (or a <b>:left</b>
      / <b>:right</b> variant).
    </p>
  </div>
  <div slot="map">
    <GeoJSON
      data={JSON.parse(notNull($model).findSeparateSidewalks())}
      generateId
    >
      <LineLayer
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": "red",
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
