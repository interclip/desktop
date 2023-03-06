<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let inputUrl = "";
  let greetMsg = "";

  async function greet() {
    greetMsg = await invoke("create_clip_cmd", { name: inputUrl });
  }
</script>

<div>
  <div class="row">
    <form action="#" on:submit={greet}>
      <input
        type="url"
        id="greet-input"
        placeholder="https://..."
        bind:value={inputUrl}
      />
      <button type="submit">Create</button>
    </form>
  </div>
  <p>{greetMsg}</p>
</div>
