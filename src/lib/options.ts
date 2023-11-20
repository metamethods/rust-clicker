import { writable } from "svelte/store";

function createOptions() {
  const options = writable<Partial<{
    delay: number;
    hotkey: number;
    clickType: "left" | "middle" | "right";
    clickCount: number;
  }>>({});

  return {
    subscribe: options.subscribe,
    set: options.set,
    update: options.update
  };
}

export default createOptions();