<script lang="ts">
  import { FillLayer, GeoJSON, LineLayer, Marker } from "svelte-maplibre";
  import ChangeSettings from "./ChangeSettings.svelte";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { Modal, SequentialLegend } from "svelte-utils";
  import { isLine, isPolygon, makeRamp, emptyGeojson } from "svelte-utils/map";
  import { isochroneMins, model, routeA, settings, settings2 } from "./stores";
  import NavBar from "./NavBar.svelte";
  import { onMount } from "svelte";
  import { colorScale } from "./colors";

  let style = "Roads";
  $: limits = Array.from(Array(6).keys()).map(
    (i) => (($isochroneMins * 60) / (6 - 1)) * i,
  );

  let compareTwo = false;
  let showTwoSettings = false;
  $: diffLimits = [
    -$isochroneMins,
    -Math.floor($isochroneMins / 2),
    -1,
    Math.floor($isochroneMins / 2),
    $isochroneMins,
  ].map((mins) => mins * 60);
  let diffColorScale = ["#a6611a", "#dfc27d", "#f5f5f5", "#80cdc1", "#018571"];

  let isochroneGj = emptyGeojson();

  onMount(() => {
    if ($routeA) {
      return;
    }

    let bbox: number[] = Array.from($model!.getBounds());
    $routeA = [lerp(0.4, bbox[0], bbox[2]), lerp(0.4, bbox[1], bbox[3])];
  });

  $: if ($routeA) {
    try {
      isochroneGj = JSON.parse(
        $model!.isochrone({
          x: $routeA[0],
          y: $routeA[1],
          style,
          time_limit: $isochroneMins,
          settings1: $settings,
          settings2: compareTwo ? $settings2 : null,
        }),
      );
    } catch (err: any) {
      console.error(err);
      isochroneGj = emptyGeojson();
    }
  }

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Isochrone mode</h2>

    <label
      >Regular
      <input type="checkbox" role="switch" bind:checked={compareTwo} />
      Compare
    </label>
    {#if compareTwo}
      <button
        class="btn btn-secondary"
        on:click={() => (showTwoSettings = true)}>Route settings</button
      >

      <Modal bind:show={showTwoSettings}>
        <div style="display: flex; gap: 150px">
          <div>
            <h2>Original</h2>
            <ChangeSettings open {settings} />
          </div>
          <div>
            <h2>Comparison</h2>
            <ChangeSettings open settings={settings2} />
          </div>
        </div>
      </Modal>
    {:else}
      <ChangeSettings open={false} {settings} />
    {/if}

    <hr />

    <label
      >Draw:
      <select bind:value={style}>
        <option value="Roads">Roads</option>
        <option value="Grid">Grid</option>
        <option value="Contours">Contours</option>
        <option value="Dasymetric">Dasymetric</option>
      </select>
    </label>

    <label
      >Minutes away
      <input type="number" bind:value={$isochroneMins} min="1" max="30" />
    </label>

    {#if compareTwo}
      <SequentialLegend
        colorScale={diffColorScale}
        labels={{ limits: diffLimits.map((l) => l / 60) }}
      />
    {:else}
      <SequentialLegend
        {colorScale}
        labels={{ limits: limits.map((l) => l / 60) }}
      />
    {/if}
  </div>
  <div slot="map">
    {#if $routeA}
      <Marker bind:lngLat={$routeA} draggable><span class="dot">X</span></Marker
      >
    {/if}

    <GeoJSON data={isochroneGj}>
      <LineLayer
        id="isochrone-roads"
        beforeId="Road labels"
        filter={isLine}
        paint={{
          "line-width": 2,
          "line-color": compareTwo
            ? makeRamp(
                ["-", ["get", "cost2"], ["get", "cost1"]],
                diffLimits,
                diffColorScale,
              )
            : makeRamp(["get", "cost1"], limits, colorScale),
        }}
      />

      <FillLayer
        id="isochrone-polygons"
        beforeId="Road labels"
        filter={isPolygon}
        paint={{
          "fill-color": compareTwo
            ? makeRamp(
                ["-", ["get", "cost2"], ["get", "cost1"]],
                diffLimits,
                diffColorScale,
              )
            : makeRamp(["get", "cost1"], limits, colorScale),
          "fill-opacity": style == "Dasymetric" ? 1.0 : 0.5,
        }}
      />
    </GeoJSON>
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    color: white;
    background-color: blue;
    font-weight: bold;
  }
</style>
