<script lang="ts">
  import { onMount } from "svelte";

  import KeyMap, { getKeyMapName } from "./keymap";

  let active = false;
  let keyName = "";

  export let key: number;

  function handleKeyDown(event: KeyboardEvent) {
    if (!active) return;
    event.preventDefault();

    if (!KeyMap[event.code]) 
      return (active = false);

    key = KeyMap[event.code];
    active = false;
  }

  $: keyName = getKeyMapName(key) ?? "";
</script>

<button 
  class:active={active} 
  class="hotkey" 
  id="hotkey" 
  on:click={() => active = !active}
>{keyName}</button>

<svelte:body on:keydown={handleKeyDown} />

<style lang="scss">
  .hotkey {
    @apply bg-zinc-900;
    @apply rounded-md;
    @apply px-2 py-1;
    @apply text-left;
    @apply outline-none;
    @apply transition;
  }

  .hotkey.active {
    @apply ring-blue-500 ring-offset-zinc-950;
    @apply ring-2 ring-offset-2;
  }
</style>