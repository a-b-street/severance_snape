<script lang="ts">
  import type { RouteGJ, Step } from "./stores";

  export let route_gj: RouteGJ;

  function levelChanges(gj: RouteGJ) {
    let count = 0;
    // No windows(2)?
    for (let i = 0; i < gj.directions.length - 1; i++) {
      let l1 = route_gj.directions[i].layer;
      let l2 = route_gj.directions[i + 1].layer;
      if (l1 != l2) {
        count++;
      }
    }
    return count;
  }

  function step(x: Step) {
    let level = parseInt(x.layer);
    let padding = "&nbsp;".repeat(3 * Math.abs(level));
    if (x.name) {
      return `${padding}[${level}] ${x.kind} (${x.name})`;
    } else {
      return `${padding}[${level}] ${x.kind}`;
    }
  }

  // TODO Move to svelte-utils
  function prettyPrintTime(seconds: number): string {
    if (seconds < 60.0) {
      return Math.round(seconds) + "s";
    }
    let hours = Math.floor(seconds / 3600);
    let minutes = Math.floor((seconds - hours * 3600) / 60);
    let leftover = Math.round(seconds - (hours * 3600) - (minutes * 60));
    if (hours > 0) {
      return `${hours}h${minutes}m${leftover}s`;
    } else {
      return `${minutes}m${leftover}s`;
    }
  }
</script>

<p>
  Detour factor: <b
    >{(route_gj.route_length / route_gj.direct_length).toFixed(1)}x</b
  > longer than straight line
</p>
<p>
  Duration: <b>{prettyPrintTime(route_gj.duration_s)}</b>
</p>
<p>{levelChanges(route_gj)} changes in level</p>

<details>
  <summary>Route directions</summary>

  <ol>
    {#each route_gj.directions as x}
      <li>
        <a href={x.way} target="_blank">{@html step(x)}</a>
      </li>
    {/each}
  </ol>
</details>
