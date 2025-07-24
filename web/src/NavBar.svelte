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

<nav class="navbar navbar-expand-lg bg-body-tertiary">
  <div class="container-fluid">
    <div class="navbar-nav">
      <button class="btn btn-secondary" on:click={titleMode}
        >Change study area</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "score" })}
        disabled={$mode.kind == "score"}>Score</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "route" })}
        disabled={$mode.kind == "route"}>Route</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "isochrone" })}
        disabled={$mode.kind == "isochrone"}>Isochrone</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "crossings" })}
        disabled={$mode.kind == "crossings"}>Crossings</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "disconnected" })}
        disabled={$mode.kind == "disconnected"}>Network disconnections</button
      >

      <button
        class="btn btn-secondary"
        on:click={() => ($mode = { kind: "debug" })}
        disabled={$mode.kind == "debug"}>Debug network</button
      >
    </div>
  </div>
</nav>
