<script lang="ts">
  import { Modal } from "svelte-utils";
  import { showAbout } from "./stores";

  // TODO The button autofocuses and scrolls things down
</script>

{#if $showAbout}
  <Modal bind:show={$showAbout}>
    <h1>Severance Snape</h1>
    <p>
      This is an <b>experimental</b> tool to study "severances" for people walking.
      In some places, crossing a big road (a "severance") might be easy -- there's
      a zebra or signalized crossing right on the "desire line" where someone might
      want to cross. But often, a person might have to walk a few blocks to reach
      the nearest crossing -- and then they might have to go up or down stairs to
      cross on a bridge or a tunnel!
    </p>
    <p>
      This tool quantifies just how bad this crossing is with a "detour factor"
      -- the ratio of the length to cross in a straight line and the length of
      the actual path, according to OSM data. A score close to 1 is great, and a
      score of 4 means the actual path is 4 times the length of the straight
      desire line.
    </p>
    <p>
      This tool is <b>experimental</b>, so there will be bugs both with OSM data
      and what the tool shows!
    </p>
    <p>To use this tool, you need to:</p>
    <ol>
      <li>Choose your study area to analyze</li>
      <li>
        Check the severances that show up -- they're inferred from OSM data and
        might not be correct
      </li>
      <li>
        Use the <b>Route mode</b> to explore the detour factor between any two points
        you choose
      </li>
      <li>
        Use the <b>Score mode</b> to visualize the detour factor everywhere, looking
        for places easy and hard to cross
      </li>
    </ol>
    <p>
      This tool aims to complement an "area porosity" analysis, defined in <a
        href="https://content.tfl.gov.uk/lcds-chapter2-toolsandtechniques.pdf"
        target="_blank">section 2.3.5 of TfL's Cycling Design Standards</a
      >. That definition of porosity just counts the number of crossings per
      area, and isn't very detailed about how far you have to walk to a crossing
      in that area.
    </p>
    <p>
      This <a
        href="https://github.com/dabreegster/severance_snape/"
        target="_blank">open source</a
      >
      tool is created by
      <a href="https://abstreet.uk" target="_blank">Dustin Carlino</a>
      and relies heavily on
      <a href="https://www.openstreetmap.org/about" target="_blank"
        >OpenStreetMap</a
      > data.
    </p>

    <details>
      <summary>Changelog</summary>
      <p>
        <a
          href="https://github.com/dabreegster/severance_snape/commits/"
          target="_blank">GitHub</a
        > has more detail; this is a summary
      </p>
      <ul>
        <li>17 February - use Pico for styling</li>
        <li>21 January - fix shortest route calculations</li>
      </ul>
    </details>

    <center
      ><button on:click={() => ($showAbout = false)}>Start!</button></center
    >
  </Modal>
{/if}
