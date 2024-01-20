<script lang="ts">
  import type { Feature } from "geojson";
  import { notNull } from "./common";
  import type { RouteGJ } from "./stores";

  export let route_gj: RouteGJ;

  function levelChanges(gj: RouteGJ) {
    let count = 0;
    // No windows(2)?
    for (let i = 0; i < gj.features.length - 1; i++) {
      let l1 = route_gj.features[i].properties!.layer ?? "0";
      let l2 = route_gj.features[i + 1].properties!.layer ?? "0";
      if (l1 != l2) {
        count++;
      }
    }
    return count;
  }

  function step(f: Feature) {
    let props = f.properties!;
    let level = props.layer ?? "0";
    let padding = "&nbsp;".repeat(3 * Math.abs(level));
    if (props.name) {
      return `${padding}[${level}] ${props.kind} (${props.name})`;
    } else {
      return `${padding}[${level}] ${props.kind}`;
    }
  }
</script>

<p>
  Detour factor: <b
    >{(route_gj.route_length / route_gj.direct_length).toFixed(1)}x</b
  > longer than straight line
</p>
<p>{levelChanges(route_gj)} changes in level</p>
<ol>
  {#each route_gj.features as f}
    <li>
      <a href={notNull(f.properties).way} target="_blank">{@html step(f)}</a>
    </li>
  {/each}
</ol>
