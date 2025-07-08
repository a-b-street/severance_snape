<script lang="ts">
  import type { Writable } from "svelte/store";
  import type { Settings } from "./stores";
  import { crossingColors } from "./colors";

  export let open: boolean;
  export let settings: Writable<Settings>;
</script>

<details {open}>
  <summary>Settings</summary>

  <label>
    <input type="checkbox" bind:checked={$settings.obey_crossings} />
    Obey crossings (otherwise, can cross severances anywhere)
  </label>

  <label>
    Base walking speed (mph)
    <input
      type="number"
      bind:value={$settings.base_speed_mph}
      min="0.1"
      max="3.5"
      step="0.1"
    />
  </label>

  <label>
    <input type="checkbox" bind:checked={$settings.use_gradient} />
    Adjust speed for gradient (<a
      href="https://en.wikipedia.org/wiki/Tobler's_hiking_function"
      target="_blank">Tobler</a
    >)
  </label>

  <label>
    <span class="circle" style:background={crossingColors.Signalized} />
    Delay at signalized crossings (s)
    <input
      type="number"
      bind:value={$settings.delay_signalized}
      min="0"
      max="60"
      step="1"
    />
  </label>

  <label>
    <span class="circle" style:background={crossingColors.Zebra} />
    Delay at zebra crossings (s)
    <input
      type="number"
      bind:value={$settings.delay_zebra}
      min="0"
      max="60"
      step="1"
    />
  </label>

  <label>
    <span class="circle" style:background={crossingColors.Other} />
    Delay at other crossings (s)
    <input
      type="number"
      bind:value={$settings.delay_other}
      min="0"
      max="60"
      step="1"
    />
  </label>
</details>

<style>
  .circle {
    height: 20px;
    width: 20px;
    border-radius: 10px;
    border: 1px solid black;
  }
</style>
