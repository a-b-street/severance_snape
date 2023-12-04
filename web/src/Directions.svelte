<script lang="ts">
  import { classifyStep } from "./logic";

  export let route_gj;

  function step(f) {
    classifyStep(f);
    let props = f.properties;
    if (props.name) {
      return `${props.type} (${props.name})`;
    } else {
      return props.type;
    }
  }
</script>

<ol>
  {#each route_gj.features as f}
    <li><a href={f.properties.way} target="_blank">{step(f)}</a></li>
  {/each}
</ol>
