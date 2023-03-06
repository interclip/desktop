<script>
	import { page } from '$app/stores';
	import Menu from '$lib/components/Menu.svelte';
</script>

<header>
	<Menu />
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}
</style>
