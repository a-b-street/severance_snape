<script lang="ts">
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import xmlUrl from "../assets/input.osm?url";
  import Layout from "./Layout.svelte";
  import Loading from "./Loading.svelte";

  let model: MapModel | undefined = undefined;
  let loading = false;

  onMount(async () => {
    await init();
    try {
      loading = true;
      let resp = await fetch(xmlUrl);
      let buffer = await resp.arrayBuffer();
      model = new MapModel(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open from URL ${xmlUrl}: ${err}`);
    }
    loading = false;
  });

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loading = true;
      let buffer = await fileInput.files![0].arrayBuffer();
      model = new MapModel(new Uint8Array(buffer));
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = false;
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <p>Canvas goes here</p>
  </div>
</Layout>
<Loading {loading} />
