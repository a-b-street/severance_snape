<script lang="ts">
  import { GeoJSON, CircleLayer } from "svelte-maplibre";
  import { crossingColors } from "./colors";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { model } from "./stores";

  export let show: boolean;
</script>

<GeoJSON data={JSON.parse(notNull($model).getCrossings())}>
  <CircleLayer
    id="crossings"
    beforeId="Road labels"
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "circle-radius": ["step", ["zoom"], 0, 12, 3, 14, 5, 15, 7],
      "circle-color": constructMatchExpression(
        ["get", "kind"],
        crossingColors,
        "red",
      ),
      "circle-stroke-color": "black",
      "circle-stroke-width": 1,
    }}
  />
</GeoJSON>
