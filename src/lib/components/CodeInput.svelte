<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import * as settings from '$lib/utils/settings';
	import { Status, type Response } from '$lib/types/api';
	import { copyIfEnabled } from '$lib/utils/copy';

	let inputCode = '';
	let output = '';
	let url = '';

	async function retrieve() {
		const endpoint = await settings.get<string>('endpoint');
		const response: Response = await invoke('retrieve_clip_cmd', {
			code: inputCode,
			options: { endpoint }
		});

		if (response.status === Status.Error) {
			output = response.result;
			return;
		} else if (response.status === Status.Success) {
			output = 'your URL: ';
			url = response.result;

			copyIfEnabled(url);
		}
	}
</script>

<div>
	<div class="row pb-8">
		<form action="#" on:submit={retrieve}>
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
	<section>
		<p>{output}</p>
		<a target="_blank" rel="noreferrer" href={url}>{url}</a>
	</section>
</div>
