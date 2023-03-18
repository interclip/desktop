<script lang="ts">
	import { get, set } from '$lib/utils/settings';

	let endpoint = 'interclip.app';
	let copyToClipboard = true;

	(async () => {
		console.debug('Loading settings');
		endpoint = (await get<string>('endpoint')) ?? 'interclip.app';
		copyToClipboard = (await get<boolean>('copyToClipboard')) ?? true;
		console.debug('Settings loaded', { endpoint, copyToClipboard });
	})();

	$: {
		console.debug('Settings saved', { endpoint, copyToClipboard });
		(async () => {
			console.debug(await get<string>('endpoint'));
		})();
	}
</script>

<main class="container">
	<h1 class="text-3xl mb-4">Settings</h1>
	<section class="flex flex-col items-left">
		<label>
			Interclip URL:
			<input
				type="url"
				bind:value={endpoint}
				on:keydown={() => {
					setTimeout(() => {
						endpoint = endpoint.trim();
						set('endpoint', endpoint);
					}, 0);
				}}
			/>
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
	</section>
</main>

<style>
</style>
