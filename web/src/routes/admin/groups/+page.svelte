<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import { checkAdvancedPerms } from '$lib/api';
</script>

<svelte:head>
	<title>RSGP - Groups</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Groups:</h1>
	<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
		><PlusIcon /></Button
	>
</div>

<div class="mx-2 flex flex-wrap gap-2">
	{#each Object.keys(data.groups) as gr}
		<Card.Root>
			<Card.Header>
				<Card.Title>{gr}</Card.Title>
			</Card.Header>
			<Card.Content>
				<h1 class="text-lg font-bold">Permissions:</h1>
				{#each data.groups[gr] as g}
					<p>{g}</p>
				{/each}
			</Card.Content>
			<Card.Footer>
				<div class="flex flex-col gap-2">
					<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
						>Change permissions</Button
					>
					<Button
						variant="destructive"
						disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
						>Delete group</Button
					>
				</div>
			</Card.Footer>
		</Card.Root>
	{/each}
</div>
