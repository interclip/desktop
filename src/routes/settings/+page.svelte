<script lang="ts">
  import { Store } from "tauri-plugin-store-api";
  const store = new Store(".settings.dat");

  //todo(ft): retrieve from store first
	let endpoint = 'https://interclip.app';
  let copyToClipboard = true;

  $: endpointHost = new URL(endpoint).host;
  $: {
    store.set("endpointHost", endpointHost );
    store.set("copyToClipboard", copyToClipboard);
  }

</script>

<main class="container">
	<h1 class="text-3xl mb-4">Settings</h1>
	<section class="flex flex-col items-left">
		<label>
			Interclip endpoint:
			<input type="text" bind:value={endpoint} />
		</label>
		<label>
			Copy to clipboard:
			<input type="checkbox" bind:value={copyToClipboard} />
		</label>
	</section>
</main>

<style>
</style>
