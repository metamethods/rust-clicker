<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  import Options from "./lib/Options.svelte";
  import Actions from "./lib/Actions.svelte";

  import active from "./lib/active";
  import options from "./lib/options";

  async function syncActive() {
    $active = await invoke<boolean>("status_clicker");
  }

  listen("click:start", async () => {
    if ($active) return;
    await invoke("clicker_start", $options);
    await syncActive();
  });

  listen("click:stop", async () => {
    if (!$active) return;
    await invoke("clicker_stop");
    await syncActive();
  });
</script>

<main>
  <Options />
  <Actions />
</main>

<style lang="scss">
  main {
    @apply flex flex-col justify-between gap-2;
    @apply p-2;
    @apply h-screen;
  }
</style>