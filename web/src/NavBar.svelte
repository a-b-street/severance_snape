<script lang="ts">
  import { mode, routeMode } from "./stores";
</script>

<nav>
  <ul>
    <li>
      <button on:click={() => ($mode = { kind: "title" })}
        >Change study area</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "score" })}
        disabled={$mode.kind == "score"}>Score</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = routeMode())}
        disabled={$mode.kind == "route"}>Route</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "debug" })}
        disabled={$mode.kind == "debug"}>Debug network</button
      >
    </li>
  </ul>
</nav>
