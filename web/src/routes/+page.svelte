<script lang="ts">
	let { data } = $props();
</script>

<svelte:head>
	<title>RSGP - Home</title>
</svelte:head>

<div class="items-center justify-center text-center text-xl">
	<h1>Hello <span class="text-primary font-bold">{data.layout!.username}</span>!</h1>
	<h2>Your servers:</h2>
</div>
