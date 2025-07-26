<script lang="ts">
	import { checkAdvancedPerms } from '$lib/api';

	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';

	let { data } = $props();

	let sc = $state(data.servers);
</script>

<svelte:head>
	<title>RSGP - Servers</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Servers:</h1>
	<Dialog.Root>
		<Dialog.Trigger>
			<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Servers', ['Write'])}>
				<PlusIcon />
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<form action="?/addserver" method="POST">
				<Dialog.Header>
					<Dialog.Title>Add new server</Dialog.Title>
					<Dialog.Description
						>This will add a new server and begin its install procedure.</Dialog.Description
					>
				</Dialog.Header>
				<div class="my-4 flex flex-col gap-2">
					<Label for="name">Name</Label>
					<Input name="name" id="name" type="text" />
					<Label for="owner">Owner</Label>
					<Input name="owner" id="owner" type="text" />
					<Label for="worker">Worker</Label>
					<Input name="worker" id="worker" type="text" />
					<Label for="egg">Egg</Label>
					<Input name="egg" id="egg" type="text" />
				</div>
				<Dialog.Footer>
					<Button type="submit">Add server</Button>
				</Dialog.Footer>
			</form>
		</Dialog.Content>
	</Dialog.Root>
</div>

<div class="mx-2 flex flex-wrap gap-2">
	{#each Object.keys(data.servers) as srv, i}
		<Card.Root>
			<Card.Header class="flex items-center justify-center gap-1">
				<Card.Title>{srv}</Card.Title>
			</Card.Header>
			<Card.Content>
				<h1 class="text-lg font-bold">Servers:</h1>
				<p>todo</p>
			</Card.Content>
			<Card.Footer>
				<div class="flex flex-col gap-2">
					<Dialog.Root>
						<Dialog.Trigger
							><Button
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Workers', ['Write'])}
								>Change password</Button
							></Dialog.Trigger
						>
						<Dialog.Content>
							<form action="?/changepassword" method="POST">
								<Dialog.Header>
									<Dialog.Title>Change password</Dialog.Title>
									<Dialog.Description>This will change the user's password.</Dialog.Description>
								</Dialog.Header>
								<div class="my-4 flex flex-col gap-2">
									<input type="text" name="user" hidden bind:value={sc[i]} />
									<Label for="password">New password</Label>
									<Input name="password" type="password" id="password" required />
								</div>
								<Dialog.Footer>
									<Button type="submit">Change password</Button>
								</Dialog.Footer>
							</form>
						</Dialog.Content>
					</Dialog.Root>
					<Dialog.Root>
						<Dialog.Trigger
							><Button
								class="w-full"
								variant="destructive"
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Servers', ['Write'])}
								>Delete worker</Button
							></Dialog.Trigger
						>
						<Dialog.Content>
							<Dialog.Header>
								<Dialog.Title>Are you absolutely sure?</Dialog.Title>
								<Dialog.Description>
									This only happens at the manager, to reset the worker, you need to delete the
									config folder there aswell.
								</Dialog.Description>
							</Dialog.Header>
							<Dialog.Footer>
								<form action="?/delete" method="POST">
									<input type="text" name="worker" id="worker" hidden bind:value={sc[i]} required />
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
