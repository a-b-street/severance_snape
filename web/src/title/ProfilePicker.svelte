<script lang="ts">
  import { profile } from "../stores";
  import { Modal } from "svelte-utils";

  let show = false;
</script>

<button class="btn btn-secondary mb-3" on:click={() => (show = true)}
  >Change import profile ({$profile})</button
>

<Modal bind:show>
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
    <div class="form-check">
      <label class="form-check-label">
        <input
          class="form-check-input"
          type="radio"
          value="SeparateWays"
          bind:group={$profile}
        />
        <u>SeparateWays</u>: if sidewalks are tagged as
        <a
          href="https://wiki.openstreetmap.org/wiki/Sidewalks#Sidewalk_as_separate_way"
          target="_blank">separate ways</a
        > in your area
      </label>
    </div>

    <div class="form-check">
      <label class="form-check-label">
        <input
          class="form-check-input"
          type="radio"
          value="SidewalksOnHighways"
          bind:group={$profile}
        />
        <u>SidewalksOnHighways</u>: When a road in OSM doesn't explicitly
        specify sidewalks, assume they exist
      </label>
    </div>

    <div class="form-check">
      <label class="form-check-label">
        <input
          class="form-check-input"
          type="radio"
          value="USA"
          bind:group={$profile}
        />
        <u>USA</u>: An attempt to simplify the classification rules, tuned for
        some USA examples. Mostly uses the <i>highway</i> tag.
      </label>
    </div>

    <div class="form-check">
      <label class="form-check-label">
        <input
          class="form-check-input"
          type="radio"
          value="USAShoulders"
          bind:group={$profile}
        />
        <u>USA with shoulders</u>: A variation that treats roads with a shoulder
        as walkable, even if their speed/classifiation would mean they're
        severances in the USA profile.
      </label>
    </div>
  </fieldset>

  <button class="btn btn-primary" on:click={() => (show = false)}>Done</button>
</Modal>
