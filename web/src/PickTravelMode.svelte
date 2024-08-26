<script lang="ts">
  import { travelMode } from "./stores";
</script>

<label
  >Mode:
  <select bind:value={$travelMode}>
    <option value="car">Car</option>
    <option value="bicycle">Bicycle</option>
    <option value="foot">Foot</option>
  </select>
</label>
