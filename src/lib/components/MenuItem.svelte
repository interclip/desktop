<script lang="ts">
	export let url: string;
	export let title: string = "";
</script>

<a href={url}>
	<li class="menu-item px-4 py-4 cursor-pointer bg-white rounded-xl dark:bg-gray-600 dark:text-white" title={title}>
		<slot />
	</li>
</a>
