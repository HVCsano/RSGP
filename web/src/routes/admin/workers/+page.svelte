<script lang="ts">
	let { data, form } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import { checkAdvancedPerms } from '$lib/api';

	let wc = $state(data.workers);
</script>

<svelte:head>
	<title>RSGP - Workers</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Workers:</h1>
	<Dialog.Root>
		<Dialog.Trigger>
			<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Workers', ['Write'])}>
				<PlusIcon />
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<form action="?/addworker" method="POST">
				<Dialog.Header>
					<Dialog.Title>Add new worker</Dialog.Title>
					<Dialog.Description>This will setup a new worker.</Dialog.Description>
				</Dialog.Header>
				<div class="my-4 flex flex-col gap-2">
					<Label for="name">Worker name</Label>
					<Input name="name" id="name" type="text" />
					<Label for="address">Address (IP or Hostname)</Label>
					<Input name="address" id="address" type="text" />
					<Label for="port">Port</Label>
					<Input name="port" id="port" type="number" />
					<div class="flex items-center justify-between">
						<Label for="https">Use https</Label>
						<Checkbox name="https" id="https" />
					</div>
				</div>
				<Dialog.Footer>
					<Button type="submit">Add user</Button>
				</Dialog.Footer>
			</form>
		</Dialog.Content>
	</Dialog.Root>
</div>

{#if form?.addworker.error}
	<div class="animate-in fade-in mx-auto mt-4 w-[400px] duration-100">
		<Alert.Root variant="destructive">
			<AlertCircleIcon />
			{#if form.addworker.error === 'checkFailed'}
				<Alert.Title>Worker check failed</Alert.Title>
				<Alert.Description>Ensure you've added the correct values.</Alert.Description>
			{/if}
			{#if form.addworker.error === 'already'}
				<Alert.Title>Worker create failed</Alert.Title>
				<Alert.Description
					>The specified worker has been already added to elsewhere.</Alert.Description
				>
			{/if}
		</Alert.Root>
	</div>
{/if}

<div class="mx-2 flex flex-wrap gap-2">
	{#each data.workers as worker, i}
		<Card.Root>
			<Card.Header>
				<Card.Title>{worker.name}</Card.Title>
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
									<input type="text" name="user" hidden bind:value={wc[i].name} />
									<Label for="password">New password</Label>
									<Input name="password" type="password" id="password" required />
									<div class="flex w-full items-center justify-between">
										<Label for="clearsessions">Log out user from everywhere</Label>
										<Checkbox name="clearsessions" id="clearsessions" />
									</div>
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
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
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
									<input
										type="text"
										name="worker"
										id="worker"
										hidden
										bind:value={wc[i].name}
										required
									/>
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
