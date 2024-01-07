<script>
	import Header from './Header.svelte';
	import './styles.css';
</script>

<div class="app flex flex-col min-h-screen">
	<Header />

	<main class="flex flex-col w-full max-w-5xl m-0 box-border justify-center items-center h-screen">
		<slot />
	</main>
</div>
