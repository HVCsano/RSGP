<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	let { data } = $props();
	let sc = $state(data.servers);
</script>

<svelte:head>
	<title>RSGP - Home</title>
</svelte:head>

<div class="items-center justify-center text-center text-xl">
	<h1>Hello <span class="text-primary font-bold">{data.layout!.username}</span>!</h1>
	<h2>Your servers:</h2>
	<div class="mt-4 flex w-full flex-col gap-4 px-2">
		{#each Object.keys(data.servers) as srv}
			<a href={`/server/${srv}`}>
				<Card.Root class="flex w-full">
					<Card.Header>
						<Card.Title>{data.servers[srv].name}</Card.Title>
					</Card.Header>
					<Card.Content>
						<p>Card Content</p>
					</Card.Content>
					<Card.Footer>
						<p>{data.servers[srv].state}</p>
					</Card.Footer>
				</Card.Root>
			</a>
		{/each}
	</div>
</div>
