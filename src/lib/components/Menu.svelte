<script>
	import MenuItem from '$lib/components/MenuItem.svelte';
	// @ts-ignore
	import SettingsIcon from '$lib/icons/settings.svg?component';
	// @ts-ignore
	import BugIcon from '$lib/icons/bug.svg?component';
</script>

<nav class="nav flex justify-between flex-row fixed top-2 px-5 w-full list-none decoration-0">
	<div class="flex gap-2">
		<MenuItem url="/">Create a clip</MenuItem>
		<MenuItem url="/receive">Get a clip</MenuItem>
	</div>
	<div class="flex gap-2">
		<MenuItem title="Show settings" url="/settings">
			<SettingsIcon />
		</MenuItem>
		<a
			title="Report a bug"
			target="_blank"
			rel="noopener noreferrer"
			href="https://github.com/interclip/iclip-tauri"
			class="menu-item px-4 py-4 cursor-pointer list-none bg-white rounded-xl dark:bg-gray-600 dark:text-white"
		>
			<BugIcon />
		</a>
	</div>
</nav>
