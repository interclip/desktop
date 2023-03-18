<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import * as settings from '$lib/utils/settings';

	let inputUrl = '';
	let output = '';

	async function greet() {
		const endpoint = await settings.get<string>('endpoint');
		output = await invoke('create_clip_cmd', { url: inputUrl, options: { endpoint } });
	}
</script>

<div>
	<div class="row pb-8">
		<form action="#" on:submit={greet}>
			<input type="url" placeholder="https://..." bind:value={inputUrl} required />
			<button type="submit">Create</button>
		</form>
	</div>
	<p>{output}</p>
</div>
