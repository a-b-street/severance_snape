<script lang="ts">
  import turfBbox from "@turf/bbox";
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre, Marker, Popup } from "svelte-maplibre";
  import inputUrl from "../assets/input.pbf?url";
  import { Layout, Legend, Loading } from "./common";
  import Directions from "./Directions.svelte";
  import NetworkLayer from "./NetworkLayer.svelte";
  import RouteLayer from "./RouteLayer.svelte";
  import ScoreLayer from "./ScoreLayer.svelte";

  let model: MapModel | undefined = undefined;
  let map;
  let loading = false;

  let mode = "route";

  let route_a = null;
  let route_b = null;
  let route_gj = null;
  let route_err = "";

  onMount(async () => {
    await init();
    try {
      loading = true;
      let resp = await fetch(inputUrl);
      loadModel(await resp.arrayBuffer());
    } catch (err) {
      window.alert(`Couldn't open from URL ${inputUrl}: ${err}`);
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
    console.time("load");
    model = new MapModel(new Uint8Array(buffer));
    console.timeEnd("load");
    zoomToFit();
    let bbox = turfBbox(JSON.parse(model.render()));
    route_a = {
      lng: lerp(0.4, bbox[0], bbox[2]),
      lat: lerp(0.4, bbox[1], bbox[3]),
    };
    route_b = {
      lng: lerp(0.6, bbox[0], bbox[2]),
      lat: lerp(0.6, bbox[1], bbox[3]),
    };
  }

  function lerp(pct, a, b) {
    return a + pct * (b - a);
  }

  function zoomToFit() {
    if (map && model) {
      // TODO wasteful
      let bbox = turfBbox(JSON.parse(model.render()));
      map.fitBounds(bbox, { animate: false });
    }
  }

  $: if (model && route_a && route_b) {
    try {
      route_gj = JSON.parse(
        model.compareRoute({
          x1: route_a.lng,
          y1: route_a.lat,
          x2: route_b.lng,
          y2: route_b.lat,
        })
      );
      route_err = "";
    } catch (err) {
      route_gj = null;
      route_err = err.toString();
    }
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
    <div><button on:click={zoomToFit}>Zoom to fit</button></div>

    <label>
      <input bind:group={mode} type="radio" value="route" />Route
    </label>
    <label>
      <input bind:group={mode} type="radio" value="score" />Score
    </label>

    {#if mode == "route"}
      <Legend
        rows={[
          ["Footway (ground, outdoors)", "red"],
          ["Indoors footway", "blue"],
          ["Footway not on the ground", "purple"],
          ["Street with sidewalk (or pedestrian street)", "black"],
          ["Crossing", "green"],
          ["Severance", "orange"],
        ]}
      />
      {#if route_err}
        <p>{route_err}</p>
      {/if}
      {#if route_gj}
        <Directions {route_gj} />
      {/if}
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={{
        version: 8,
        sources: {},
        layers: [],
      }}
      standardControls
      hash
      bind:map
    >
      {#if model}
        {#if mode == "route"}
          <NetworkLayer {model} />
          <RouteLayer bind:route_a bind:route_b {route_gj} />
        {:else if mode == "score"}
          <ScoreLayer {model} />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>
<Loading {loading} />
