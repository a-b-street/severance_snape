<script lang="ts">
  import turfBbox from "@turf/bbox";
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre, Marker, Popup } from "svelte-maplibre";
  import xmlUrl from "../assets/input.osm?url";
  import Directions from "./Directions.svelte";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
  import Loading from "./Loading.svelte";
  import { classifyStep } from "./logic";

  let model: MapModel | undefined = undefined;
  let map;
  let loading = false;

  let route_a = null;
  let route_b = null;
  let route_gj = null;
  let route_err = "";

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

  function renderNetwork() {
    let gj = JSON.parse(model.render());
    // Easier to add props here than attempt style expressions
    for (let f of gj.features) {
      classifyStep(f);
    }
    return gj;
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

  export function constructMatchExpression<OutputType>(
    getter: any[],
    map: { [name: string]: OutputType },
    fallback: OutputType
  ): DataDrivenPropertyValueSpecification<OutputType> {
    let x: any[] = ["match", getter];
    for (let [key, value] of Object.entries(map)) {
      x.push(key);
      x.push(value);
    }
    x.push(fallback);
    return x as DataDrivenPropertyValueSpecification<OutputType>;
  }
</script>

<Layout>
  <div slot="left">
    <label>
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
    <div><button on:click={zoomToFit}>Zoom to fit</button></div>
    <Legend
      rows={[
        ["Footway (ground, outdoors)", "red"],
        ["Indoors footway", "blue"],
        ["Footway not on the ground", "purple"],
        ["Street with sidewalk (or pedestrian street)", "black"],
        ["Crossing", "green"],
      ]}
    />
    {#if route_err}
      <p>{route_err}</p>
    {/if}
    {#if route_gj}
      <Directions {route_gj} />
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
    >
      {#if model}
        <GeoJSON data={renderNetwork()}>
          <LineLayer
            id="network"
            paint={{
              "line-width": 5,
              "line-color": constructMatchExpression(
                ["get", "type"],
                {
                  footway: "red",
                  "indoors footway": "blue",
                  "footway not on the ground": "purple",
                  sidewalk: "black",
                  crossing: "green",
                },
                "orange"
              ),
            }}
            on:click={(e) =>
              window.open(e.detail.features[0].properties.way, "_blank")}
            hoverCursor="pointer"
          >
            <Popup openOn="hover" let:data
              >{@html JSON.stringify(data.properties, null, "<br />")}</Popup
            >
          </LineLayer>
        </GeoJSON>
        {#if route_a}
          <Marker bind:lngLat={route_a} draggable
            ><span class="dot">A</span></Marker
          >
          <Marker bind:lngLat={route_b} draggable
            ><span class="dot">B</span></Marker
          >
        {/if}
        {#if route_gj}
          <GeoJSON data={route_gj}>
            <LineLayer
              id="route"
              beforeId="network"
              paint={{
                "line-width": 20,
                "line-color": "cyan",
                "line-opacity": 0.5,
              }}
            />
          </GeoJSON>
        {/if}
        <GeoJSON data={JSON.parse(model.renderSeverances())}>
          <LineLayer
            id="severances"
            paint={{
              "line-width": 5,
              "line-color": "orange",
            }}
            on:click={(e) =>
              window.open(e.detail.features[0].properties.way, "_blank")}
            hoverCursor="pointer"
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

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: inline-block;
    background-color: grey;
    text-align: center;
  }
</style>
