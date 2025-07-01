<script lang="ts">
  import { Control } from "svelte-maplibre";
  import {
    gradientLimits,
    gradientColors,
    kindToColor,
    crossingColors,
  } from "./colors";
  import { SequentialLegend, QualitativeLegend } from "svelte-utils";
  import chevron from "../assets/chevron.png?url";

  export let zoomToFit: () => void;
  export let opacity: number;
  export let showCrossings: boolean;
  export let canShowCrossings: boolean;
  export let showGradient: boolean;
  export let canShowGradient: boolean;
</script>

<Control position="top-right">
  <div style:background="white" style:width="200px" style:padding="8px">
    <details open>
      <summary>Layers</summary>
      <button class="outline" style="margin-bottom: 8px" on:click={zoomToFit}
        >Zoom to fit</button
      >

      <QualitativeLegend
        labelColors={{
          Footway: kindToColor.Footway,
          Crossing: kindToColor["Crossing(Signalized)"],
          "Walkable and easily crossable street (maybe no sidewalk)":
            kindToColor.WithTraffic,
          Severance: kindToColor.Severance,
        }}
        itemsPerRow={1}
      />
      <label style="margin-top: 8px">
        Network opacity:
        <input type="range" min="0" max="100" bind:value={opacity} />
      </label>

      {#if canShowCrossings}
        <label>
          <input type="checkbox" bind:checked={showCrossings} />
          Crossings
        </label>
        {#if showCrossings}
          <QualitativeLegend
            labelColors={crossingColors}
            itemsPerRow={1}
            swatchClass="circle"
          />
        {/if}
      {/if}

      {#if canShowGradient}
        <label>
          <input type="checkbox" bind:checked={showGradient} />
          Gradient
        </label>
        {#if showGradient}
          <SequentialLegend
            colorScale={gradientColors}
            labels={{ limits: gradientLimits }}
          />
          <p>
            <img src={chevron} alt="arrow" />
            Arrows point uphill
          </p>
        {/if}
      {/if}
    </details>
  </div>
</Control>
