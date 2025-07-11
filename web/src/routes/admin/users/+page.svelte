<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { checkAdvancedPerms } from '$lib/api';

	let users_clone = $state(Object.keys(data.users));
</script>

<svelte:head>
	<title>RSGP - Users</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Users:</h1>
	<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
		><PlusIcon /></Button
	>
</div>

<div class="mx-2 flex flex-wrap gap-2">
	{#each Object.keys(data.users) as user, i}
		<Card.Root>
			<Card.Header>
				<Card.Title>{user}</Card.Title>
			</Card.Header>
			<Card.Content>
				<h1 class="text-lg font-bold">Groups:</h1>
				{#each data.users[user].groups as g}
					<p>{g}</p>
				{/each}
			</Card.Content>
			<Card.Footer>
				<div class="flex flex-col gap-2">
					<Dialog.Root>
						<Dialog.Trigger
							><Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
								>Change password</Button
							></Dialog.Trigger
						>
						<Dialog.Content>
							<Dialog.Header>
								<Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
								<Dialog.Description>
									This action cannot be undone. This will permanently delete your account and remove
									your data from our servers.
								</Dialog.Description>
							</Dialog.Header>
							<Dialog.Footer>
								<Button type="submit">Save changes</Button>
							</Dialog.Footer>
						</Dialog.Content>
					</Dialog.Root>
					<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
						>Change group</Button
					>
					<Dialog.Root>
						<Dialog.Trigger
							><Button
								class="w-full"
								variant="destructive"
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
								>Delete user</Button
							></Dialog.Trigger
						>
						<Dialog.Content>
							<Dialog.Header>
								<Dialog.Title>Are you absolutely sure?</Dialog.Title>
								<Dialog.Description>
									This action cannot be undone. This will permanently delete the user and remove all
									sessions.
								</Dialog.Description>
							</Dialog.Header>
							<Dialog.Footer>
								<form action="?/deleteuser" method="POST">
									<input type="text" name="user" id="user" hidden bind:value={users_clone[i]} />
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
