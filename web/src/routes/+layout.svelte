<script lang="ts">
	import { page } from '$app/state';
	import '../app.css';

	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	import OwnSidebar from '$lib/ui/sidebar.svelte';

	import { ModeWatcher } from 'mode-watcher';

	let { children, data } = $props();
</script>

<svelte:head>
	<title>Rust Game Panel</title>
	<meta name="description" content="A simple game panel written in Rust." />
	<meta content="Rust Game Panel" property="og:title" />
	<meta content="/" property="og:url" />
	<meta content="A simple game panel written in Rust." property="og:description" />
	<meta content="/favicon.png" property="og:image" />
	<meta content="#CE422B" data-react-helmet="true" name="theme-color" />
</svelte:head>

<ModeWatcher />

{#if page.url.pathname !== '/login'}
	<header>
		<Sidebar.Provider>
			<OwnSidebar
				permissions={data.layout!.permissions}
				admin={data.layout!.permissions.includes('AdminPage') ||
					data.layout!.permissions.includes('Admin')}
			/>
			<main class="w-full">
				<Sidebar.Trigger />
				<div class="w-full text-center">
					{@render children?.()}
				</div>
			</main>
		</Sidebar.Provider>
	</header>
{:else}
	{@render children?.()}
{/if}

<footer>
	<h1 class="absolute bottom-2 right-2">
		Made by <a class="font-bold text-[#76ff7a]" target="_blank" href="https://csano.hu">HVCsano</a>,
		with ❤️
	</h1>
</footer>
