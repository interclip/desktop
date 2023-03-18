<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import * as settings from '$lib/utils/settings';

	let inputCode = '';
	let output = '';

	async function greet() {
		const endpoint = await settings.get<string>('endpoint');
		output = await invoke('retrieve_clip_cmd', { code: inputCode, options: { endpoint } });
	}
</script>

<div>
	<div class="row pb-8">
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
