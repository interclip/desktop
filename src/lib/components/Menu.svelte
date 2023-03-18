<script>
	import MenuItem from '$lib/components/MenuItem.svelte';
	import { bugIcon, settingsIcon } from '$lib/utils/icons';
</script>

<nav class="nav flex justify-between flex-row fixed top-2 px-5 w-full list-none decoration-0">
	<div class="flex gap-2">
		<MenuItem url="/">Create a clip</MenuItem>
		<MenuItem url="/receive">Get a clip</MenuItem>
	</div>
	<div class="flex gap-2">
		<MenuItem title="Show settings" url="/settings">
			{@html settingsIcon}
		</MenuItem>
		<a
			title="Report a bug"
			target="_blank"
			rel="noopener noreferrer"
			href="https://github.com/interclip/desktop/issues/new"
			class="menu-item px-4 py-4 cursor-pointer list-none bg-white rounded-xl dark:bg-gray-600 dark:text-white"
		>
			{@html bugIcon}
		</a>
	</div>
</nav>
