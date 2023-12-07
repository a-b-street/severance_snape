<script lang="ts">
  import { GeoJSON, LineLayer, Popup } from "svelte-maplibre";
  import { constructMatchExpression } from "./common";

  export let model;
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
    }}
    on:click={(e) => window.open(e.detail.features[0].properties.way, "_blank")}
    hoverCursor="pointer"
  >
    <Popup openOn="hover" let:data
      >{@html JSON.stringify(data.properties, null, "<br />")}</Popup
    >
  </LineLayer>
</GeoJSON>
