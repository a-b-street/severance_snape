<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import {
    PropertiesTable,
    notNull,
    downloadGeneratedFile,
  } from "svelte-utils";
  import { Popup, constructMatchExpression } from "svelte-utils/map";
  import { overpassQueryForPolygon } from "svelte-utils/overpass";
  import { model } from "./stores";
  import NavBar from "./NavBar.svelte";

  // TODO Could try to reuse NetworkLayer and add in the interactivity

  export let opacity: number;

  async function updateOsm() {
    // TODO The original clipping boundary isn't preserved. This will grow the
    // area size every use.
    let gj = JSON.parse($model!.getInvertedBoundary());
    // Un-invert the boundary
    let boundary = gj.geometry.coordinates.pop();
    gj.geometry.coordinates = [boundary];

    let resp = await fetch(overpassQueryForPolygon(gj));
    let xml = await resp.text();
    downloadGeneratedFile("updated_osm.xml", xml);
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Debug mode</h2>
    <p>Hover to see a segment's properties, and click to open OSM</p>

    <button on:click={updateOsm}>Update OSM data</button>
  </div>
  <div slot="map">
    <GeoJSON data={JSON.parse(notNull($model).render())} generateId>
      <LineLayer
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": constructMatchExpression(
            ["get", "kind"],
            kindToColor,
            "yellow",
          ),
          "line-opacity": opacity / 100,
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
