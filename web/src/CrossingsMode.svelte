<script lang="ts">
  import {
    GeoJSON,
    CircleLayer,
    LineLayer,
    hoverStateFilter,
  } from "svelte-maplibre";
  import {
    Checkbox,
    notNull,
    PropertiesTable,
    SequentialLegend,
  } from "svelte-utils";
  import { Popup, makeRamp } from "svelte-utils/map";
  import { model } from "./stores";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import NavBar from "./NavBar.svelte";
  import { colorScale } from "./colors";

  let crossingsGj = JSON.parse($model!.getCrossings());
  let includeKinds = getKinds();
  $: filterKinds = Object.keys(includeKinds).filter((key) => includeKinds[key]);

  function getKinds(): Record<string, boolean> {
    let kinds: Record<string, boolean> = {};
    for (let f of crossingsGj.features) {
      let key: string = f.properties.crossing || "unknown";
      kinds[key] = true;
    }
    return kinds;
  }

  let limits = [1, 100, 200, 400, 800, 5000];
</script>

<SplitComponent>
  <div slot="top"><NavBar /></div>
  <div slot="sidebar">
    <h2>Crossings mode</h2>
    <p>
      This is only showing crossings over severances. For other streets, it's
      assumed that it's easy enough to cross the street anywhere, even without
      explicit crossings.
    </p>

    <SequentialLegend {colorScale} labels={{ limits }} />

    <hr />
    <p>What crossings do you want to include for measuring distance?</p>

    {#each Object.keys(includeKinds) as key}
      <Checkbox bind:checked={includeKinds[key]}>
        {key}
        {#if key != "unknown"}
          <a
            href="https://wiki.openstreetmap.org/wiki/Tag:crossing%3D{key}"
            target="_blank">?</a
          >
        {/if}
      </Checkbox>
    {/each}
  </div>
  <div slot="map">
    <GeoJSON
      data={JSON.parse(notNull($model).getCrossingDistances(filterKinds))}
      generateId
    >
      <LineLayer
        beforeId="Road labels"
        paint={{
          "line-width": hoverStateFilter(5, 10),
          "line-color": makeRamp(["get", "length"], limits, colorScale),
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          {Math.round(props.length)}m
        </Popup>
      </LineLayer>
    </GeoJSON>

    <GeoJSON data={crossingsGj}>
      <CircleLayer
        paint={{
          "circle-radius": ["step", ["zoom"], 0, 12, 3, 14, 5, 15, 7],
          "circle-color": [
            "case",
            [
              "in",
              ["coalesce", ["get", "crossing"], "unknown"],
              ["literal", filterKinds],
            ],
            "yellow",
            "grey",
          ],
          "circle-stroke-color": "black",
          "circle-stroke-width": 1,
        }}
        hoverCursor="pointer"
      >
        <Popup openOn="click" let:props let:features>
          {@const [lon, lat] = features[0].geometry.coordinates}
          <PropertiesTable properties={props} />
          <a href={props.url} target="_blank">OSM</a>,
          <a
            href={`http://maps.google.com/maps?q=&layer=c&cbll=${lat},${lon}&cbp=11,0,0,0,0`}
            target="_blank">Google StreetView</a
          >,
          <a
            href={`https://www.bing.com/maps?cp=${lat}~${lon}&style=x`}
            target="_blank">Bing Streetside</a
          >
        </Popup>
      </CircleLayer>
    </GeoJSON>
  </div>
</SplitComponent>
