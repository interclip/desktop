<script lang="ts">
	import { get, set } from '$lib/utils/settings';

	let endpoint = 'interclip.app';
	let copyToClipboard = false;
	let pasteOnLoad = false;

	let output = '';

	const save = () => {
		setTimeout(() => {
			endpoint = endpoint.trim();
			try {
				const url = new URL(endpoint);
				output = '';
				set('endpoint', `${url.protocol}//${url.host}`);
			} catch (e) {
				output = 'Invalid URL';
			}
		}, 0);
	};

	(async () => {
		console.debug('Loading settings');
		endpoint = (await get<string>('endpoint')) ?? 'interclip.app';
		copyToClipboard = (await get<boolean>('copyToClipboard')) ?? true;
		pasteOnLoad = (await get<boolean>('pasteOnLoad')) ?? true;
		console.debug('Settings loaded', { endpoint, copyToClipboard });
	})();

	$: {
		console.debug('Settings saved', { endpoint, copyToClipboard });
	}
</script>

<main class="container">
	<h1 class="text-3xl mb-4">Settings</h1>
	<section class="flex flex-col items-left">
		<label>
			Interclip URL:
			<input type="url" bind:value={endpoint} on:blur={save} on:keydown={save} />
		</label>
		<label>
			Copy to clipboard:
			<input
				type="checkbox"
				bind:checked={copyToClipboard}
				on:change={(e) => {
					//@ts-ignore
					if (e?.target instanceof HTMLInputElement) {
						copyToClipboard = e?.target.checked;
					}
					set('copyToClipboard', copyToClipboard);
				}}
			/>
		</label>
		<label>
			Paste from clipboard automatically:
			<input
				type="checkbox"
				bind:checked={pasteOnLoad}
				on:change={(e) => {
					//@ts-ignore
					if (e?.target instanceof HTMLInputElement) {
						pasteOnLoad = e?.target.checked;
					}
					set('pasteOnLoad', pasteOnLoad);
				}}
			/>
		</label>
		{#if output}
			<p>{output}</p>
		{/if}
	</section>
</main>

<style>
</style>
