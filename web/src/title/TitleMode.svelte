<script lang="ts">
  import { Modal, notNull } from "../common";
  import PolygonToolLayer from "../common/draw_polygon/PolygonToolLayer.svelte";
  import SplitComponent from "../SplitComponent.svelte";
  import { map } from "../stores";
  import MapLoader from "./MapLoader.svelte";

  export let wasmReady: boolean;

  // TODO Once per session
  let showModal = true;
</script>

<SplitComponent>
  <div slot="sidebar">
    {#if showModal}
      <Modal on:close={() => (showModal = false)} let:dialog>
        <h1>Severance Snape</h1>
        <p>
          This is an <b>experimental</b> tool to study "severances" for people walking.
          In some places, crossing a big road (a "severance") might be easy -- there's
          a zebra or signalized crossing right on the "desire line" where someone
          might want to cross. But often, a person might have to walk a few blocks
          to reach the nearest crossing -- and then they might have to go up or down
          stairs to cross on a bridge or a tunnel!
        </p>
        <p>
          This tool quantifies just how bad this crossing is with a "detour
          factor" -- the ratio of the length to cross in a straight line and the
          length of the actual path, according to OSM data. A score close to 1
          is great, and a score of 4 means the actual path is 4 times the length
          of the straight desire line.
        </p>
        <p>
          This tool is <b>experimental</b>, so there will be bugs both with OSM
          data and what the tool shows!
        </p>
        <p>To use this tool, you need to:</p>
        <ol>
          <li>Choose your study area to analyze</li>
          <li>
            Check the severances that show up -- they're inferred from OSM data
            and might not be correct
          </li>
          <li>
            Use the <b>Route mode</b> to explore the detour factor between any two
            points you choose
          </li>
          <li>
            Use the <b>Score mode</b> to visualize the detour factor everywhere,
            looking for places easy and hard to cross
          </li>
        </ol>
        <p>
          This tool aims to complement an "area porosity" analysis, defined in <a
            href="https://content.tfl.gov.uk/lcds-chapter2-toolsandtechniques.pdf"
            target="_blank">section 2.3.5 of TfL's Cycling Design Standards</a
          >. That definition of porosity just counts the number of crossings per
          area, and isn't very detailed about how far you have to walk to a
          crossing in that area.
        </p>
        <p>
          This tool is created by <a
            href="https://github.com/dabreegster/"
            target="_blank">Dustin Carlino</a
          >
          and relies heavily on
          <a href="https://www.openstreetmap.org/about" target="_blank"
            >OpenStreetMap</a
          > data.
        </p>
        <center
          ><button on:click={() => notNull(dialog).close()}>Start!</button
          ></center
        >
      </Modal>
    {/if}

    <h1>Choose your study area</h1>
    <button on:click={() => (showModal = true)}>About the crossing tool</button>
    <hr />

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
