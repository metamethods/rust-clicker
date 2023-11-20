<script lang="ts">
  import { onMount } from "svelte";

  import options from "./options";

  function clamp(value: number, minimum: number, maximum: number) {
    return Math.min(Math.max(value, minimum), maximum);
  }

  function getValue<T=string>(id: string): T {
    return (document.getElementById(id) as HTMLInputElement).value as T;
  }

  function parameterChanged() {
    const delayNumberValue = Number(getValue("delay"));

    const delay = clamp(Math.floor(delayNumberValue), 0, Number.MAX_SAFE_INTEGER);
    const clickType = getValue("clickType") as "left" | "middle" | "right";
    const clickCount = clamp(Math.floor(Number(getValue("clickCount"))), 1, 100);

    options.set({ delay, clickType, clickCount });
  }

  onMount(() => {
    parameterChanged();
  });
</script>

<form on:change|preventDefault={parameterChanged} on:submit|preventDefault={()=>{}}>
  <section>
    <label for="hotkey">Hotkey</label>
    <button class="hotkey" id="hotkey" placeholder="Hotkey">F6</button>
  </section>

  <section>
    <label for="delay">Delay</label>
    <input id="delay" type="number" placeholder="milliseconds" value="100" />
  </section>

  <section>
    <label for="delay">Click Type</label>
    <select id="clickType">
      <option value="left">Left Click</option>
      <option value="middle">Middle Click</option>
      <option value="right">Right Click</option>
    </select>
  </section>

  <section>
    <label for="delay">Click Count</label>
    <input id="clickCount" type="number" placeholder="clicks" value="1" />
  </section>
</form>


<style lang="scss">
  form {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: .5rem;
    
    & > section {
      @apply flex flex-col gap-1;
      @apply w-full;
    }
  }

  .hotkey {
    @apply bg-zinc-900/50;
    @apply rounded-md;
    @apply px-2 py-1;
    @apply text-left;
    @apply cursor-auto;
  }

  // Removes the up/down arrows from number inputs
  input::-webkit-outer-spin-button,
  input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  input[type="number"] {
    -webkit-appearance: textfield;
    -moz-appearance: textfield;
    appearance: textfield;
  }
</style>