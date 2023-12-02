<script lang="ts">
  import turfBbox from "@turf/bbox";
  import init, { MapModel } from "backend";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre, Popup } from "svelte-maplibre";
  import xmlUrl from "../assets/input.osm?url";
  import Layout from "./Layout.svelte";
  import Legend from "./Legend.svelte";
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

  function renderNetwork() {
    let gj = JSON.parse(model.render());
    // Easier to add props here than attempt style expressions
    for (let f of gj.features) {
      let props = f.properties;
      if (
        props.highway == "crossing" ||
        props.footway == "crossing" ||
        "crossing" in props
      ) {
        props.color = "green";
      } else if (props.highway == "footway") {
        // TODO The categories aren't mutex, some could combo
        if (props.indoor) {
          props.color = "blue";
        } else if (props.layer || props.bridge || props.tunnel) {
          props.color = "purple";
        } else {
          props.color = "red";
        }
      } else {
        props.color = "black";
      }
    }
    return gj;
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
            paint={{
              "line-width": 5,
              "line-color": ["get", "color"],
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
