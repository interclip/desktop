<script lang="ts">
	import { onMount } from 'svelte';

	export let url;
</script>

<a href={url}>
	<li class="menu-item px-4 py-4 cursor-pointer bg-white rounded-xl dark:bg-gray-600 dark:text-white">
		<slot />
	</li>
</a>
