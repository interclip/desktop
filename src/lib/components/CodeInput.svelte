<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
  
    let inputCode = "";
    let output = "";
  
    async function greet() {
      output = await invoke("retrieve_clip_cmd", { code: inputCode });
    }
  </script>
  
  <div>
    <div class="row">
      <form action="#" on:submit={greet}>
        <input
          type="text"
          placeholder=""
          bind:value={inputCode}
          maxlength="5"
          minlength="5"
          required
        />
        <button type="submit">Receive</button>
      </form>
    </div>
    <p>{output}</p>
  </div>
  