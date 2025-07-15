<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import { checkAdvancedPerms } from '$lib/api';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let gc = $state(Object.keys(data.groups));
</script>

<svelte:head>
	<title>RSGP - Groups</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Groups:</h1>
	<Dialog.Root>
		<Dialog.Trigger>
			<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}>
				<PlusIcon />
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<form action="?/addgroup" method="POST">
				<Dialog.Header>
					<Dialog.Title>Add new group</Dialog.Title>
					<Dialog.Description>This will create a new group with no permissions.</Dialog.Description>
				</Dialog.Header>
				<div class="my-4 flex flex-col gap-2">
					<Label for="name">Group name</Label>
					<Input name="name" id="name" type="text" />
				</div>
				<Dialog.Footer>
					<Button type="submit">Add group</Button>
				</Dialog.Footer>
			</form>
		</Dialog.Content>
	</Dialog.Root>
</div>

<div class="mx-2 flex flex-wrap gap-2">
	{#each Object.keys(data.groups) as gr, i}
		<Card.Root>
			<Card.Header>
				<Card.Title>{gr}</Card.Title>
			</Card.Header>
			<Card.Content>
				<h1 class="text-lg font-bold">Permissions:</h1>
				{#each data.groups[gr] as g}
					{#if typeof g == 'string'}
						<p>{g}</p>
					{:else}
						<p>{Object.keys(g)[0]}: {g[Object.keys(g)[0]]}</p>
					{/if}
				{/each}
			</Card.Content>
			<Card.Footer>
				<div class="flex flex-col gap-2">
					<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
						>Change permissions</Button
					>
					<Dialog.Root>
						<Dialog.Trigger
							><Button
								class="w-full"
								variant="destructive"
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
								>Delete group</Button
							></Dialog.Trigger
						>
						<Dialog.Content>
							<Dialog.Header>
								<Dialog.Title>Are you absolutely sure?</Dialog.Title>
								<Dialog.Description>
									This action cannot be undone. This will permanently delete the group.
								</Dialog.Description>
							</Dialog.Header>
							<Dialog.Footer>
								<form action="?/deletegroup" method="POST">
									<input type="text" name="name" id="name" hidden bind:value={gc[i]} />
									<Button variant="destructive" type="submit">Delete</Button>
								</form>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
				</div>
			</Card.Footer>
		</Card.Root>
	{/each}
</div>
