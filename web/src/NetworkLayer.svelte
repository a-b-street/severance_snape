<script lang="ts">
  import buffer from "@turf/buffer";
  import { GeoJSON, FillLayer } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { model } from "./stores";

  export let show: boolean;
  // TODO Use filter expressions?
  export let showSeverances: boolean;
  export let opacity: number;

  function thicken(gj) {
    let crossings = [];
    let rest = [];
    for (let f of gj.features) {
      let radius;
      if (f.properties.kind == "Severance") {
        radius = 10;
      } else if (f.properties.kind == "Crossing") {
        radius = 5;
      } else {
        radius = 2;
      }
      let thicker = buffer(f, radius / 1000);
      f.geometry = thicker.geometry;

      if (f.properties.kind == "Crossing") {
        crossings.push(f);
      } else {
        rest.push(f);
      }
    }
    gj.features = [...rest, ...crossings];
    return gj;
  }
</script>

<GeoJSON data={thicken(JSON.parse(notNull($model).render()))}>
  <FillLayer
    id="network"
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "fill-color": constructMatchExpression(
        ["get", "kind"],
        {
          Crossing: "green",
        },
        "white",
      ),
      "fill-opacity": showSeverances
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
