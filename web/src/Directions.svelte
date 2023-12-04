<script lang="ts">
  import { classifyStep } from "./logic";

  export let route_gj;

  function step(f) {
    classifyStep(f);
    let props = f.properties;
    let level = props.layer ?? "0";
    let padding = "&nbsp;".repeat(3 * Math.abs(level));
    if (props.name) {
      return `${padding}[${level}] ${props.type} (${props.name})`;
    } else {
      return `${padding}[${level}] ${props.type}`;
    }
  }
</script>

<ol>
  {#each route_gj.features as f}
    <li><a href={f.properties.way} target="_blank">{@html step(f)}</a></li>
  {/each}
</ol>
