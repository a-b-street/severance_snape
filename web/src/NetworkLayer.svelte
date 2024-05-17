<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { model } from "./stores";

  // TODO Use filter expressions?
  export let showSeverances: boolean;
  export let opacity: number;
</script>

<GeoJSON data={JSON.parse(notNull($model).render())}>
  <LineLayer
    id="network"
    paint={{
      "line-width": 5,
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
  />
</GeoJSON>
