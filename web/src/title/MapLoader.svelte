<script lang="ts">
  import { MapModel } from "backend";
  import { onMount } from "svelte";
  import { Loading, OverpassSelector } from "../common";
  import { importStreetsWithoutSidewalkTagging, map, model } from "../stores";

  let example = "";
  let msg: string | null = null;
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
  });

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    try {
      loadModel(await fileInput.files![0].arrayBuffer());
      example = "";
    } catch (err) {
      window.alert(`Couldn't open this file: ${err}`);
    }
    msg = null;
  }

  function loadModel(buffer: ArrayBuffer) {
    msg = "Building map model from OSM input";
    console.time("load");
    $model = new MapModel(
      new Uint8Array(buffer),
      $importStreetsWithoutSidewalkTagging,
    );
    console.timeEnd("load");
  }

  function gotXml(e: CustomEvent<string>) {
    try {
      // TODO Can we avoid turning into bytes?
      loadModel(new TextEncoder().encode(e.detail));
      example = "";
    } catch (err) {
      window.alert(`Couldn't import from Overpass: ${err}`);
    }
    msg = null;
  }

  async function loadExample(example: string) {
    if (example != "") {
      if (useLocalVite) {
        await loadFromUrl(`/${example}.pbf`);
      } else {
        await loadFromUrl(
          `https://assets.od2net.org/severance_pbfs/${example}.pbf`,
        );
      }
    }
  }
  $: loadExample(example);

  async function loadFromUrl(url: string) {
    try {
      msg = `Downloading ${url}`;
      let resp = await fetch(url);
      loadModel(await resp.arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open from URL ${url}: ${err}`);
    }
    msg = null;
  }
</script>

<Loading {msg} />

<div style="border: 1px solid black; padding: 8px;">
  <div>
    <label>
      Load an example:
      <select bind:value={example} on:change={() => loadExample(example)}>
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
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>

  <i>or...</i>

  <OverpassSelector
    map={$map}
    on:gotXml={gotXml}
    on:loading={(e) => (msg = e.detail)}
    on:error={(e) => window.alert(e.detail)}
  />
</div>
