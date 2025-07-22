<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { checkAdvancedPerms } from '$lib/api';

	let multiselect: string[] = $state([]);

	let users_clone = $state(Object.keys(data.users));

	function changegrouptrigger(user: string) {
		multiselect = data.users[user].groups;
	}
</script>

<svelte:head>
	<title>RSGP - Users</title>
</svelte:head>

<div class="flex items-center justify-center gap-4">
	<h1 class="text-3xl font-bold">Users:</h1>
	<Dialog.Root>
		<Dialog.Trigger>
			<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}>
				<PlusIcon />
			</Button>
		</Dialog.Trigger>
		<Dialog.Content>
			<form action="?/adduser" method="POST">
				<Dialog.Header>
					<Dialog.Title>Add new user</Dialog.Title>
					<Dialog.Description>This will create a new user with no groups.</Dialog.Description>
				</Dialog.Header>
				<div class="my-4 flex flex-col gap-2">
					<Label for="username">Username</Label>
					<Input name="username" id="username" type="text" required />
					<Label for="password">Password</Label>
					<Input name="password" id="password" type="password" required />
				</div>
				<Dialog.Footer>
					<Button type="submit">Add user</Button>
				</Dialog.Footer>
			</form>
		</Dialog.Content>
	</Dialog.Root>
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
							<form action="?/changepassword" method="POST">
								<Dialog.Header>
									<Dialog.Title>Change password</Dialog.Title>
									<Dialog.Description>This will change the user's password.</Dialog.Description>
								</Dialog.Header>
								<div class="my-4 flex flex-col gap-2">
									<input type="text" name="user" hidden bind:value={users_clone[i]} />
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
						<Dialog.Trigger onclick={() => changegrouptrigger(user)}>
							<Button
								class="w-full"
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
								>Change groups</Button
							>
						</Dialog.Trigger>
						<Dialog.Content>
							<form action="?/changegroups" method="POST">
								<Dialog.Header>
									<Dialog.Title>Change groups</Dialog.Title>
									<Dialog.Description>Change the users groups</Dialog.Description>
								</Dialog.Header>
								<div class="my-4 flex flex-col gap-2">
									<div class="flex items-center justify-between">
										<Select.Root bind:value={multiselect} type="multiple">
											<Select.Trigger class="w-[180px]">{multiselect}</Select.Trigger>
											<Select.Content>
												{#each data.groups as group}
													<Select.Item value={group}>{group}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
										<input hidden name="groups" type="text" bind:value={multiselect} />
										<input type="text" name="user" id="user" hidden bind:value={users_clone[i]} />
									</div>
								</div>
								<Dialog.Footer>
									<Button type="submit">Save changes</Button>
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
