<script lang="ts">
	import { getThemeContext } from '$lib/theme';
	import { Home, Moon, Sun } from 'lucide-svelte';
	import type { ActivityItem } from '../../types';
	import ActivityIcon from './ActivityIcon.svelte';

	let {
		activeSidebar,
		onSidebarChange,
		topItems,
		bottomItems
	}: {
		activeSidebar: string;
		onSidebarChange: (id: string) => void;
		topItems: ActivityItem[];
		bottomItems: ActivityItem[];
	} = $props();

	const themeCtx = getThemeContext();
</script>

<nav
	data-tauri-drag-region
	class="bg-activitybar-bg border-activitybar-border flex w-12 shrink-0 flex-col items-center justify-between border-r py-2"
>
	<div data-tauri-drag-region class="flex w-full flex-col gap-1">
		<a
			href="/"
			title="Home"
			class="text-activitybar-inactive-fg hover:text-activitybar-fg relative flex h-12 w-full items-center justify-center no-underline opacity-80 transition-all hover:opacity-100"
		>
			<Home size={24} strokeWidth={1.5} />
		</a>
		<div class="bg-activitybar-border mx-auto my-0.5 h-px w-6"></div>
		{#each topItems as item (item.id)}
			<ActivityIcon
				icon={item.icon}
				active={activeSidebar === item.id}
				tooltip={item.tooltip}
				onclick={() => onSidebarChange(item.id)}
			/>
		{/each}
	</div>
	<div class="mb-1 flex w-full flex-col gap-1">
		{#each bottomItems as item (item.id)}
			<ActivityIcon
				icon={item.icon}
				tooltip={item.tooltip}
				active={activeSidebar === item.id}
				onclick={() => onSidebarChange(item.id)}
			/>
		{/each}
		<ActivityIcon
			icon={themeCtx.resolved === 'dark' ? Sun : Moon}
			tooltip="Toggle Theme"
			active={false}
			onclick={() => themeCtx.toggleTheme()}
		/>
	</div>
</nav>
