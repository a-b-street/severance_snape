<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    Marker,
    Popup,
  } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { constructMatchExpression, notNull, PropertiesTable } from "./common";
  import Directions from "./Directions.svelte";
  import SplitComponent from "./SplitComponent.svelte";
  import { map, mode, model, type RouteGJ } from "./stores";

  // TODO Use filter expressions?
  export let showSeverances: boolean;
  export let opacity: number;

  // TODO Maybe need to do this when model changes
  let bbox = Array.from($model!.getBounds());
  let route_a = {
    lng: lerp(0.4, bbox[0], bbox[2]),
    lat: lerp(0.4, bbox[1], bbox[3]),
  };
  let route_b = {
    lng: lerp(0.6, bbox[0], bbox[2]),
    lat: lerp(0.6, bbox[1], bbox[3]),
  };

  // TODO or empty
  let route_gj: RouteGJ | null = null;
  let route_err = "";

  $: if (route_a && route_b) {
    try {
      route_gj = JSON.parse(
        $model!.compareRoute({
          x1: route_a.lng,
          y1: route_a.lat,
          x2: route_b.lng,
          y2: route_b.lat,
        })
      );
      route_err = "";
    } catch (err: any) {
      route_gj = null;
      route_err = err.toString();
    }
  }

  onMount(() => {
    $map?.on("contextmenu", onRightClick);
  });
  onDestroy(() => {
    $map?.off("contextmenu", onRightClick);
  });
  function onRightClick(e: MapMouseEvent) {
    // Move the first marker, for convenience
    route_a = e.lngLat;
  }

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h1>Route mode</h1>
    <div>
      <button on:click={() => ($mode = "title")}>Change study area</button>
      <button on:click={() => ($mode = "score")}>Score mode</button>
    </div>
    <p>Move the <b>A</b> and <b>B</b> pins to find a walking route</p>
    {#if route_err}
      <p>{route_err}</p>
    {/if}
    {#if route_gj}
      <Directions {route_gj} />
    {/if}
  </div>
  <div slot="map">
    <GeoJSON data={JSON.parse(notNull($model).render())} generateId>
      <LineLayer
        id="network"
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": constructMatchExpression(
            ["get", "kind"],
            kindToColor,
            "yellow"
          ),
          "line-opacity": showSeverances
            ? opacity / 100
            : constructMatchExpression(
                ["get", "kind"],
                {
                  Severance: 0.0,
                },
                opacity / 100.0
              ),
        }}
        manageHoverState
        on:click={(e) =>
          window.open(notNull(e.detail.features[0].properties).way, "_blank")}
        hoverCursor="pointer"
      >
        <Popup openOn="hover" let:data>
          <PropertiesTable properties={notNull(data).properties} />
        </Popup>
      </LineLayer>
    </GeoJSON>

    <Marker bind:lngLat={route_a} draggable><span class="dot">A</span></Marker>
    <Marker bind:lngLat={route_b} draggable><span class="dot">B</span></Marker>
    {#if route_gj}
      <GeoJSON data={route_gj}>
        <LineLayer
          id="route"
          beforeId="network"
          paint={{
            "line-width": 20,
            "line-color": "cyan",
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: inline-block;
    background-color: grey;
    text-align: center;
  }
</style>
