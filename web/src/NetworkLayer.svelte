<script lang="ts">
  import { MapModel } from "backend";
  import { GeoJSON, hoverStateFilter, LineLayer, Popup } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { constructMatchExpression, PropertiesTable } from "./common";
  import { model } from "./stores";

  // TODO Use filter expressions?
  export let showSeverances: boolean;
  export let opacity: number;
</script>

<GeoJSON data={JSON.parse($model.render())} generateId>
  <LineLayer
    id="network"
    paint={{
      "line-width": hoverStateFilter(5, 7),
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
    manageHoverState
    on:click={(e) => window.open(e.detail.features[0].properties.way, "_blank")}
    hoverCursor="pointer"
  >
    <Popup openOn="hover" let:data>
      <PropertiesTable properties={data.properties} />
    </Popup>
  </LineLayer>
</GeoJSON>
