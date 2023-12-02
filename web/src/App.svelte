<script lang="ts">
  import turfBbox from "@turf/bbox";
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre, Popup } from "svelte-maplibre";
  import xmlUrl from "../assets/input.osm?url";
  import Layout from "./Layout.svelte";
  import Loading from "./Loading.svelte";

  let model: MapModel | undefined = undefined;
  let map;
  let loading = false;

  onMount(async () => {
    await init();
    try {
      loading = true;
      let resp = await fetch(xmlUrl);
      loadModel(await resp.arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open from URL ${xmlUrl}: ${err}`);
    }
    loading = false;
  });

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      loadModel(await fileInput.files![0].arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = false;
  }

  function loadModel(buffer: Buffer) {
    model = new MapModel(new Uint8Array(buffer));
  }

  function zoomToFit() {
    if (map && model) {
      // TODO wasteful
      let bbox = turfBbox(JSON.parse(model.render()));
      map.fitBounds(bbox, { animate: false });
    }
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
    <div><button on:click={zoomToFit}>Zoom to fit</button></div>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      {#if model}
        <GeoJSON data={JSON.parse(model.render())}>
          <LineLayer
            paint={{
              "line-width": 5,
              "line-color": "red",
            }}
            on:click={(e) =>
              window.open(e.detail.features[0].properties.way, "_blank")}
          >
            <Popup openOn="hover" let:data
              >{@html JSON.stringify(data.properties, null, "<br />")}</Popup
            >
          </LineLayer>
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>
<Loading {loading} />
