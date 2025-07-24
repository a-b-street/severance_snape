<script lang="ts">
  import { SymbolLayer, GeoJSON, LineLayer } from "svelte-maplibre";
  import { gradientLimits, gradientColors, kindToColor } from "./colors";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression, makeRamp } from "svelte-utils/map";
  import { model } from "./stores";

  export let show: boolean;
  export let opacity: number;
  export let showGradient: boolean;
</script>

<GeoJSON data={JSON.parse(notNull($model).render())}>
  <LineLayer
    id="network"
    beforeId="Road labels"
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": 5,
      "line-color": showGradient
        ? makeRamp(["abs", ["get", "gradient"]], gradientLimits, gradientColors)
        : constructMatchExpression(["get", "kind"], kindToColor, "yellow"),
      "line-opacity": opacity / 100,
    }}
  />

  <SymbolLayer
    id="gradient-arrows"
    beforeId="Road labels"
    minzoom={12}
    filter={[">", ["abs", ["get", "gradient"]], 3]}
    layout={{
      "icon-image": "chevron",
      "icon-size": 1.0,
      "symbol-placement": "line",
      "symbol-spacing": 50,
      "icon-allow-overlap": true,
      "icon-rotate": ["case", ["<", ["get", "gradient"], 0], 180, 0],
      visibility: show && showGradient ? "visible" : "none",
    }}
  />
</GeoJSON>
