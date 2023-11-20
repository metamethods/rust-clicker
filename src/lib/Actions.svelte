<script lang="ts">
  import { emit } from "@tauri-apps/api/event";

  import options from "./options";
  import active from "./active";

  let disabledClass = "opacity-50 cursor-not-allowed";
</script>

<div class="actions">
  <button
    class="{$active ? disabledClass : ""}"
    on:click={()=>emit("click:start")}
  >Start</button>

  <button
    class="{!$active ? disabledClass : ""}"
    on:click={()=>emit("click:stop")}
  >Stop</button>
</div>

<style class="scss">
  button {
    @apply bg-zinc-900;
    @apply rounded-md;
    @apply px-4 py-3;
    @apply w-full;
  }

  .actions {
    @apply flex justify-between gap-2;
  }
</style>