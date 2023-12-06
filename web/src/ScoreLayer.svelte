<script lang="ts">
  import { GeoJSON, LineLayer, Popup } from "svelte-maplibre";

  export let model;
</script>

<GeoJSON data={JSON.parse(model.makeHeatmap())}>
  <LineLayer
    id="scores"
    paint={{
      "line-width": 5,
      "line-color": "red",
    }}
  >
    <Popup openOn="hover" let:data
      >{@html JSON.stringify(data.properties, null, "<br />")}</Popup
    >
  </LineLayer>
</GeoJSON>
