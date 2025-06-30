<script lang="ts">
  import { settings } from "./stores";
</script>

<details open>
  <summary>Settings</summary>

  <label>
    <input type="checkbox" bind:checked={$settings.obey_crossings} />
    Obey crossings (otherwise, can cross severances anywhere)
  </label>

  <label>
    Base walking speed (mph)
    <input
      type="number"
      bind:value={$settings.base_speed}
      min="0.1"
      max="3.5"
      step="0.1"
    />
  </label>
</details>
