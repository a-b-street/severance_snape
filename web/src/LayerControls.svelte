<script lang="ts">
  import { Control } from "svelte-maplibre";
  import { kindToColor } from "./colors";
  import { QualitativeLegend } from "svelte-utils";

  export let zoomToFit: () => void;
  export let opacity: number;
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
          Crossing: kindToColor.Crossing,
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
    </details>
  </div>
</Control>
