<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import * as settings from '$lib/utils/settings';
	import { Status, type Response } from '$lib/types/api';
	import { copyIfEnabled } from '$lib/utils/copy';

	let inputUrl = '';
	let output = '';
	let code = '';

	async function create() {
		const endpoint = await settings.get<string>('endpoint');
		const response: Response = await invoke('create_clip_cmd', {
			url: inputUrl,
			options: { endpoint }
		});

		if (response.status === Status.Error) {
			output = response.result;
			return;
		} else if (response.status === Status.Success) {
			output = 'your code: ';
			code = response.result;

			copyIfEnabled(code);
		}
	}
</script>

<div>
	<div class="row pb-8">
		<form action="#" on:submit={create}>
			<input type="url" placeholder="https://..." bind:value={inputUrl} required autofocus />
			<button type="submit">Create</button>
		</form>
	</div>
	<section>
		<p>{output}</p>
		<pre>{code}</pre>
	</section>
</div>
