<script lang="ts">
  import { mode } from "./stores";
</script>

<nav>
  <ul>
    <li>
      <button on:click={() => ($mode = "title")}>Change study area</button>
    </li>

    <li>
      <button on:click={() => ($mode = "score")} disabled={$mode == "score"}
        >Score</button
      >
    </li>

    <li>
      <button on:click={() => ($mode = "route")} disabled={$mode == "route"}
        >Route</button
      >
    </li>

    <li>
      <button on:click={() => ($mode = "debug")} disabled={$mode == "debug"}
        >Debug network</button
      >
    </li>
  </ul>
</nav>
