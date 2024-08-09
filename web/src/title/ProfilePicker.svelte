<script lang="ts">
  import { profile } from "../stores";
  import { Modal } from "svelte-utils";

  let show = false;
</script>

<button on:click={() => (show = true)}
  >Change import profile ({$profile})</button
>

{#if show}
  <Modal on:close={() => (show = false)}>
    <h1>Import profile</h1>

    <p>
      Street design and OpenStreetMap tagging conventions both vary around the
      world. This tool needs to identify "severance" roads that're difficult to
      cross. Pick a profile below (or <a
        href="https://github.com/a-b-street/severance_snape/issues/9"
        target="_blank">help make a new one</a
      >).
    </p>

    <fieldset>
      <label>
        <input type="radio" value="SeparateWays" bind:group={$profile} />
        <u>SeparateWays</u>: if sidewalks are tagged as
        <a
          href="https://wiki.openstreetmap.org/wiki/Sidewalks#Sidewalk_as_separate_way"
          target="_blank">separate ways</a
        > in your area
      </label>

      <label>
        <input type="radio" value="SidewalksOnHighways" bind:group={$profile} />
        <u>SidewalksOnHighways</u>: When a road in OSM doesn't explicitly
        specify sidewalks, assume they exist
      </label>

      <label>
        <input type="radio" value="USA" bind:group={$profile} />
        <u>USA</u>: An attempt to simplify the classification rules, tuned for
        some USA examples. Mostly uses the <i>highway</i> tag.
      </label>
    </fieldset>

    <button on:click={() => (show = false)}>Done</button>
  </Modal>
{/if}
