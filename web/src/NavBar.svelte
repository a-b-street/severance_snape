<script lang="ts">
  import { mode, routeMode } from "./stores";

  function titleMode() {
    let url = new URL(window.location.href);
    url.searchParams.delete("study_area");
    window.history.replaceState(null, "", url.toString());

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
        on:click={() => ($mode = routeMode())}
        disabled={$mode.kind == "route"}>Route</button
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
        on:click={() => ($mode = { kind: "osm-separate-sidewalks" })}
        disabled={$mode.kind == "osm-separate-sidewalks"}
        >Fix OSM sidewalk=separate</button
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
