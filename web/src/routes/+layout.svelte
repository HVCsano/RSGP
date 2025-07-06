<script lang="ts">
	import '../app.css';

	import { ModeWatcher } from 'mode-watcher';

	let { children } = $props();
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

{@render children()}

<footer>
	<h1 class="absolute bottom-2 left-2">
		Made by <a class="font-bold text-[#76ff7a]" target="_blank" href="https://csano.hu">HVCsano</a>,
		with ❤️
	</h1>
</footer>
