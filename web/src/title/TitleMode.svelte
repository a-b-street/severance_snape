<script lang="ts">
  import { PolygonToolLayer } from "maplibre-draw-polygon";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { map, model } from "../stores";
  import MapLoader from "./MapLoader.svelte";
  import ProfilePicker from "./ProfilePicker.svelte";

  export let wasmReady: boolean;

  // When other modes reset here, they can't clear the model without a race condition
  $model = null;
</script>

<SplitComponent>
  <div slot="top"></div>
  <div slot="sidebar">
    <h2>Choose your study area</h2>

    <ProfilePicker />

    {#if $map && wasmReady}
      <MapLoader />
    {:else}
      <p>Waiting for MapLibre and WASM to load...</p>
    {/if}
  </div>

  <div slot="map">
    <PolygonToolLayer />
  </div>
</SplitComponent>
