<script lang="ts">
  import { MapModel } from "backend";
  import { onMount } from "svelte";
  import { Loading } from "svelte-utils";
  import { OverpassSelector } from "svelte-utils/overpass";
  import { profile, map, model } from "../stores";
  import type { Feature, Polygon } from "geojson";

  let setupDone = false;
  let example = "";
  let loading = "";
  let useLocalVite = false;
  let exampleAreas: [string, [string, string][]][] = [];

  onMount(async () => {
    // When running locally if a vite public/ directory is set up, load from that for speed
    try {
      let resp = await fetch("/osm/areas.json");
      if (resp.ok) {
        useLocalVite = true;
        console.log("Using local cache, not od2net.org");
        exampleAreas = await resp.json();
      } else {
        let resp = await fetch(
          `https://assets.od2net.org/severance_pbfs/areas.json`,
        );
        exampleAreas = await resp.json();
      }

      // For quicker dev
      //example = "kowloon";
    } catch (err) {}

    // Auto-restore from URL
    let param = new URLSearchParams(window.location.search).get("study_area");
    if (param) {
      // No need to validate -- if it doesn't exist, error handling will show it later anyway
      example = param;
    }
    setupDone = true;
  });

  let fileInput1: HTMLInputElement;
  async function loadOSMFile(e: Event) {
    try {
      loadFromOSM(await fileInput1.files![0].arrayBuffer());
      example = "";
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = "";
  }

  let fileInput2: HTMLInputElement;
  async function loadBinFile(e: Event) {
    try {
      loading = "Deserializing binary file";
      let buffer = await fileInput2.files![0].arrayBuffer();
      console.time("load");
      let isOSM = false;
      $model = new MapModel(isOSM, new Uint8Array(buffer), $profile);
      console.timeEnd("load");
      example = "";
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    loading = "";
  }

  function loadFromOSM(buffer: ArrayBuffer) {
    loading = "Building map model from OSM input";
    console.time("load");
    let isOSM = true;
    $model = new MapModel(isOSM, new Uint8Array(buffer), $profile);
    console.timeEnd("load");
  }

  function gotXml(e: CustomEvent<{ xml: string; boundary: Feature<Polygon> }>) {
    try {
      // TODO Can we avoid turning into bytes?
      loadFromOSM(new TextEncoder().encode(e.detail.xml));
      example = "";
    } catch (err) {
      window.alert(`Couldn't import from Overpass: ${err}`);
    }
    loading = "";
  }

  async function loadExample(example: string, setupDone: boolean) {
    if (!setupDone) {
      return;
    }
    let url = new URL(window.location.href);

    if (example == "") {
      url.searchParams.delete("study_area");
      window.history.replaceState(null, "", url.toString());
      return;
    }

    url.searchParams.set("study_area", example);
    window.history.replaceState(null, "", url.toString());
    if (useLocalVite) {
      await loadFromUrl(`/osm/${example}.pbf`);
    } else {
      await loadFromUrl(
        `https://assets.od2net.org/severance_pbfs/${example}.pbf`,
      );
    }
  }
  $: loadExample(example, setupDone);

  async function loadFromUrl(url: string) {
    try {
      loading = `Downloading ${url}`;
      let resp = await fetch(url);
      loadFromOSM(await resp.arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open from URL ${url}: ${err}`);
    }
    loading = "";
  }
</script>

<Loading {loading} />

<div>
  <label>
    Load an example:
    <select bind:value={example}>
      <option value="">Custom file loaded</option>
      {#each exampleAreas as [country, areas]}
        <optgroup label={country}>
          {#each areas as [value, label]}
            <option {value}>{label}</option>
          {/each}
        </optgroup>
      {/each}
    </select>
  </label>
</div>

<i>or...</i>

<div>
  <label>
    Load an osm.xml or a .pbf file:
    <input bind:this={fileInput1} on:change={loadOSMFile} type="file" />
  </label>
</div>

<i>or...</i>

<OverpassSelector
  map={$map}
  on:gotXml={gotXml}
  on:loading={(e) => (loading = e.detail)}
  on:error={(e) => window.alert(e.detail)}
/>

<i>or...</i>

<div>
  <label>
    Load a pre-built .bin file:
    <input bind:this={fileInput2} on:change={loadBinFile} type="file" />
  </label>
</div>
