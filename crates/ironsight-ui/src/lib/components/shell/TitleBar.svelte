<script lang="ts">
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { Search, Layout, PanelBottom, ChevronDown, Minus, Square, X } from 'lucide-svelte';
	import { page } from '$app/state';
	import Dialog from '$components/overlay/Dialog.svelte';
	import { qrsWebSocket } from '$stores/qrs-websocket.svelte';
	import KeybindingLabel from '$lib/components/display/KeybindingLabel.svelte';

	export type NavMenuItem = {
		label: string;
		href: string;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		icon?: any;
	};

	export type MenuCategory = {
		label: string;
		items: NavMenuItem[];
	};

	let {
		onToggleSidebar,
		onToggleTerminal,
		onToggleMobileMenu,
		workspaceName,
		menuItems
	}: {
		onToggleSidebar: () => void;
		onToggleTerminal: () => void;
		onToggleMobileMenu: () => void;
		workspaceName: string;
		menuItems: MenuCategory[];
	} = $props();

	let openMenuIndex = $state<number | null>(null);
	let hoverIntent = $state(false);

	let aiChatOpen = $state(false);
	let aiChatLoading = $state(false);
	let aiChatResponse = $state('');
	let aiChatError = $state('');

	// ── Close Confirmation Modal ─────────────────────────
	let closeConfirmationOpen = $state(false);

	// ── Tauri window controls ────────────────────────────
	let isMaximized = $state(false);

	$effect(() => {
		const appWindow = getCurrentWindow();
		appWindow.isMaximized().then((m) => {
			isMaximized = m;
		});

		const unlisten = appWindow.onResized(async () => {
			isMaximized = await appWindow.isMaximized();
		});

		return () => {
			unlisten.then((fn) => fn());
		};
	});

	async function minimize() {
		await getCurrentWindow().minimize();
	}

	async function toggleMaximize() {
		await getCurrentWindow().toggleMaximize();
	}

	async function close() {
		closeConfirmationOpen = true;
	}

	function confirmClose(result: {
		button: string;
		inputValues: string[];
		checkboxChecked: boolean;
	}) {
		if (result.button === 'Quit') {
			getCurrentWindow().close();
		}
	}

	// ── AI Chat via WebSocket ────────────────────────────
	$effect(() => {
		const unsub = qrsWebSocket.onMessage((msg) => {
			if (!msg || typeof msg !== 'object') return;
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			const p = msg as any;
			if (p.channel === 'ai.chat') {
				if (p.event === 'chat_token' && p.data) {
					aiChatResponse += p.data.delta ?? '';
					if (p.data.done) {
						aiChatLoading = false;
					}
				} else if (p.event === 'chat_error' && p.data) {
					aiChatError = p.data.error ?? 'Unknown error';
					aiChatLoading = false;
				}
			}
		});
		return () => unsub();
	});

	function handleGlobalKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			aiChatOpen = true;
		}
	}

	function handleAiChatSubmit(result: {
		button: string;
		inputValues: string[];
		checkboxChecked: boolean;
	}) {
		if (result.button === 'Send') {
			const prompt = result.inputValues[0];
			if (prompt && prompt.trim()) {
				aiChatLoading = true;
				aiChatResponse = '';
				aiChatError = '';
				qrsWebSocket.send({
					channel: 'ai.chat',
					event: 'chat_request',
					data: { prompt }
				});

				return false; // prevent immediate close
			}
		} else {
			aiChatLoading = false;
			aiChatResponse = '';
			aiChatError = '';
			// returns undefined -> closes automatically
		}
	}

	function handleMenuEnter(index: number) {
		openMenuIndex = index;
		hoverIntent = true;
	}

	function handleMenuLeave() {
		hoverIntent = false;
		setTimeout(() => {
			if (!hoverIntent) openMenuIndex = null;
		}, 150);
	}

	function handleDropdownEnter() {
		hoverIntent = true;
	}

	function handleDropdownLeave() {
		hoverIntent = false;
		setTimeout(() => {
			if (!hoverIntent) openMenuIndex = null;
		}, 150);
	}

	function handleItemClick() {
		openMenuIndex = null;
		hoverIntent = false;
	}

	function isActive(href: string): boolean {
		const path = page.url.pathname;
		if (href === '/') return path === '/';
		return path === href || path.startsWith(href + '/');
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<header
	class="bg-titlebar-bg text-titlebar-fg border-titlebar-border z-50 flex h-10 shrink-0 items-center justify-between border-b px-3 select-none"
>
	<div data-tauri-drag-region class="flex items-center gap-4">
		<div data-tauri-drag-region class="flex items-center gap-3 text-[12px] text-inherit opacity-80">
			<svg class="h-4 w-4 text-blue-500 pointer-events-none" viewBox="0 0 24 24" fill="currentColor">
				<path
					d="M23.5 15.4l-3.8-3.8 3.8-3.9c.4-.4.4-1 0-1.4l-2.1-2.1c-.4-.4-1-.4-1.4 0l-5.1 5.1-3-3-1.1 1.1 3 3-5.3 5.3-3.2-3.2-1.1 1.1 3.2 3.2-1.9 1.9c-.4.4-.4 1 0 1.4l2.1 2.1c.4.4 1 .4 1.4 0l3.8-3.8 3.8 3.8c.4.4 1 .4 1.4 0l2.1-2.1c.5-.4.5-1.1.1-1.5z"
				/>
			</svg>
			<div class="flex gap-0">
				{#each menuItems as category, i (category.label)}
					<div
						class="menu-trigger relative"
						role="menubar"
						tabindex="0"
						onmouseenter={() => handleMenuEnter(i)}
						onmouseleave={handleMenuLeave}
					>
						<button data-tauri-drag-region="false"
							class="hover:bg-foreground/10 m-0 flex cursor-pointer items-center gap-0.5 rounded border-none bg-transparent px-2 py-0.5 text-[12px] text-inherit transition-colors
								{openMenuIndex === i ? 'bg-foreground/10' : ''}"
						>
							{category.label}
							<ChevronDown class="h-2.5 w-2.5 opacity-50 pointer-events-none" />
						</button>

						{#if openMenuIndex === i}
							<div
								class="menu-dropdown border-menu-border bg-menu-bg absolute top-full left-0 z-999 mt-px min-w-[220px] rounded-md border py-1 shadow-2xl"
								role="menu"
								tabindex="-1"
								onmouseenter={handleDropdownEnter}
								onmouseleave={handleDropdownLeave}
							>
								{#each category.items as item (item.href)}
									<a
										href={item.href}
										onclick={handleItemClick}
										class="hover:bg-menu-selection hover:text-menu-selection-fg flex items-center gap-2.5 px-3 py-[5px] text-[12px] no-underline transition-colors
											{isActive(item.href)
											? 'bg-background-selected text-foreground-bright font-medium'
											: 'text-menu-fg'}"
									>
										{#if item.icon}
											{@const Icon = item.icon}
											<Icon class="h-3.5 w-3.5 shrink-0 opacity-70 pointer-events-none" />
										{/if}
										<span class="truncate">{item.label}</span>
									</a>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>

	<div data-tauri-drag-region class="relative mx-4 max-w-xl flex-1 flex h-full items-center">
		<button data-tauri-drag-region="false"
			onclick={() => (aiChatOpen = true)}
			class="bg-foreground/5 hover:bg-foreground/10 border-titlebar-border relative m-0 flex h-6 w-full cursor-pointer items-center justify-center gap-2 rounded border text-[11px] text-inherit opacity-60 transition-colors hover:opacity-100"
		>
			<Search class="h-3 w-3 pointer-events-none" />
			<span>{workspaceName}</span>
			<div class="pointer-events-none absolute right-1.5 flex items-center">
				<KeybindingLabel keybinding="Cmd+K" os="mac" />
			</div>
		</button>
	</div>

	<div class="flex items-center gap-4 opacity-80 h-full">
		<div class="flex items-center gap-3 text-inherit h-full">
			<button data-tauri-drag-region="false"
				class="m-0 cursor-pointer border-none bg-transparent p-0 text-inherit"
				aria-label="Layout"
				onclick={onToggleSidebar}
			>
				<Layout class="hover:text-foreground h-4 w-4 pointer-events-none" />
			</button>
			<button data-tauri-drag-region="false"
				class="m-0 block cursor-pointer border-none bg-transparent p-0 text-inherit"
				aria-label="Terminal"
				onclick={onToggleTerminal}
			>
				<PanelBottom class="h-4 w-4 hover:text-white pointer-events-none" />
			</button>
		</div>

		<!-- Window controls -->
		<div class="flex h-full items-center">
			<button data-tauri-drag-region="false"
				class="flex h-8 w-9 items-center justify-center border-none bg-transparent text-inherit transition-colors hover:bg-white/10"
				onclick={minimize}
				aria-label="Minimize"
			>
				<Minus class="h-3.5 w-3.5 pointer-events-none" />
			</button>
			<button data-tauri-drag-region="false"
				class="flex h-8 w-9 items-center justify-center border-none bg-transparent text-inherit transition-colors hover:bg-white/10"
				onclick={toggleMaximize}
				aria-label={isMaximized ? 'Restore' : 'Maximize'}
			>
				{#if isMaximized}
					<svg class="h-3 w-3 pointer-events-none" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2">
						<rect x="3" y="5" width="8" height="8" rx="0.5" />
						<path d="M5 5V3.5a.5.5 0 0 1 .5-.5H13.5a.5.5 0 0 1 .5.5V11.5a.5.5 0 0 1-.5.5H11" />
					</svg>
				{:else}
					<Square class="h-3 w-3 pointer-events-none" />
				{/if}
			</button>
			<button data-tauri-drag-region="false"
				class="flex h-8 w-9 items-center justify-center border-none bg-transparent text-inherit transition-colors hover:bg-red-500/90 hover:text-white"
				onclick={close}
				aria-label="Close"
			>
				<X class="h-3.5 w-3.5 pointer-events-none" />
			</button>
		</div>
	</div>
</header>

<Dialog
	bind:open={closeConfirmationOpen}
	title="Quit IronSight EDR?"
	message="Are you sure you want to quit the application? Any active sessions will be closed."
	buttons={[
		{ label: 'Cancel', variant: 'secondary' },
		{ label: 'Quit', variant: 'primary' }
	]}
	onsubmit={confirmClose}
/>

<Dialog
	bind:open={aiChatOpen}
	title="AI Chat"
	message="Ask the AI a question or give it a prompt."
	inputs={[{ placeholder: 'Type your message...', type: 'text' }]}
	buttons={[
		{ label: 'Cancel', variant: 'secondary' },
		{ label: 'Send', variant: 'primary' }
	]}
	loading={aiChatLoading}
	onsubmit={handleAiChatSubmit}
>
	{#if aiChatError}
		<div
			class="border-error-border bg-error-bg/10 text-error-fg mt-4 rounded border p-3 text-[12px]"
		>
			{aiChatError}
		</div>
	{:else if aiChatResponse}
		<div
			class="border-foreground/10 bg-foreground/5 text-foreground mt-4 max-h-[300px] overflow-y-auto rounded border p-3 font-mono text-[12px] whitespace-pre-wrap"
		>
			{aiChatResponse}
		</div>
	{/if}
</Dialog>
