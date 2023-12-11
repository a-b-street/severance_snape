<script lang="ts">
  import turfBbox from "@turf/bbox";
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import inputUrl from "../assets/input.pbf?url";
  import { Loading, OverpassSelector } from "./common";

  export let model: MapModel | undefined = undefined;
  export let map;

  let msg = null;

  onMount(async () => {
    await init();
    try {
      msg = `Downloading ${inputUrl}`;
      let resp = await fetch(inputUrl);
      loadModel(await resp.arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open from URL ${inputUrl}: ${err}`);
    }
    msg = null;
  });

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loadModel(await fileInput.files![0].arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    msg = null;
  }

  function loadModel(buffer: Buffer) {
    msg = "Building map model from OSM input";
    console.time("load");
    model = new MapModel(new Uint8Array(buffer));
    console.timeEnd("load");
  }

  function gotXml(e: CustomEvent<string>) {
    try {
      // TODO Can we avoid turning into bytes?
      loadModel(new TextEncoder().encode(e.detail));
    } catch (err) {
      window.alert(`Couldn't import from Overpass: ${err}`);
    }
    msg = null;
  }
</script>

<Loading {msg} />

<div style="border: 1px solid black; padding: 8px;">
  <div>
    <label>
      Load an osm.xml or a .pbf file:
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>

  <OverpassSelector
    {map}
    on:gotXml={gotXml}
    on:loading={(e) => (msg = e.detail)}
    on:error={(e) => window.alert(e.detail)}
  />
</div>
