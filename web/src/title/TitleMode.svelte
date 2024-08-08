<script lang="ts">
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { importStreetsWithoutSidewalkTagging, map, model } from "../stores";
  import MapLoader from "./MapLoader.svelte";

  export let wasmReady: boolean;

  // When other modes reset here, they can't clear the model without a race condition
  $model = null;
</script>

<SplitComponent>
  <div slot="top"></div>
  <div slot="sidebar">
    <h2>Choose your study area</h2>

    {#if $map && wasmReady}
      <MapLoader />
    {:else}
      <p>Waiting for MapLibre and WASM to load...</p>
    {/if}

    <div>
      <label>
        <input
          type="checkbox"
          bind:checked={$importStreetsWithoutSidewalkTagging}
        />
        When a road in OSM doesn't explicitly specify sidewalks, assume they exist
        or not? (This tool works best in places where sidewalks are consistently
        tagged as
        <a
          href="https://wiki.openstreetmap.org/wiki/Sidewalks#Sidewalk_as_separate_way"
          target="_blank">separate ways</a
        >. Disable this in those places.)
      </label>
    </div>
  </div>

  <div slot="map">
    <PolygonToolLayer />
  </div>
</SplitComponent>
