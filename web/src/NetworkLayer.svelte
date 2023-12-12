<script lang="ts">
  import { MapModel } from "backend";
  import { GeoJSON, LineLayer, Popup } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { constructMatchExpression } from "./common";

  export let model: MapModel;
  // TODO Use filter expressions?
  export let showSeverances: boolean;
  export let opacity: number;
</script>

<GeoJSON data={JSON.parse(model.render())}>
  <LineLayer
    id="network"
    paint={{
      "line-width": 5,
      "line-color": constructMatchExpression(
        ["get", "kind"],
        kindToColor,
        "yellow"
      ),
      "line-opacity": showSeverances
        ? opacity / 100
        : constructMatchExpression(
            ["get", "kind"],
            {
              Severance: 0.0,
            },
            opacity / 100.0
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
