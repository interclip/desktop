<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    let inputUrl = "";
    let output = "";
  
    async function greet() {
      output = await invoke("create_clip_cmd", { url: inputUrl });
    }
  </script>
  
  <div>
    <div class="row pb-8">
      <form action="#" on:submit={greet}>
        <input
          type="url"
          placeholder="https://..."
          bind:value={inputUrl}
          required
        />
        <button type="submit">Create</button>
      </form>
    </div>
    <p>{output}</p>
  </div>
  