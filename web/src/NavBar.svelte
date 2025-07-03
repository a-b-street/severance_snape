<script lang="ts">
  import { mode, routeA, routeB } from "./stores";

  function titleMode() {
    let url = new URL(window.location.href);
    url.searchParams.delete("study_area");
    window.history.replaceState(null, "", url.toString());

    $routeA = null;
    $routeB = null;
    $mode = { kind: "title" };
  }
</script>

<nav>
  <ul>
    <li>
      <button on:click={titleMode}>Change study area</button>
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "score" })}
        disabled={$mode.kind == "score"}>Score</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "route" })}
        disabled={$mode.kind == "route"}>Route</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "isochrone" })}
        disabled={$mode.kind == "isochrone"}>Isochrone</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "crossings" })}
        disabled={$mode.kind == "crossings"}>Crossings</button
      >
    </li>

    <li>
      <button
        on:click={() => ($mode = { kind: "disconnected" })}
        disabled={$mode.kind == "disconnected"}>Network disconnections</button
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
