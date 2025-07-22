<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import { checkAdvancedPerms } from '$lib/api';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let gc = $state(Object.keys(data.groups));
	let modifiers = ['none', 'read', 'write'];
	type modifiersType = 'none' | 'read' | 'write';
	let permissions: {
		login: boolean;
		admin: boolean;
		adminpage: boolean;
		user: modifiersType;
		users: modifiersType;
		groups: modifiersType;
		servers: modifiersType;
		workers: modifiersType;
		eggs: modifiersType;
		sitesettings: modifiersType;
	} = $state({
		login: false,
		admin: false,
		adminpage: false,
		eggs: 'none',
		groups: 'none',
		servers: 'none',
		sitesettings: 'none',
		user: 'none',
		users: 'none',
		workers: 'none'
	});
	function loadPerms(group: string) {
		let g = data.groups[group];
		for (let i = 0; i < g.length; i++) {
			let obj = g[i];
			if (typeof obj === 'string') {
				if (obj === 'Login') permissions.login = true;

				if (obj === 'AdminPage') permissions.adminpage = true;

				if (obj === 'admin') permissions.admin = true;
			} else if (typeof obj === 'object') {
				if (Object.keys(obj)[0] === 'User') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.user = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.user = 'write';
				}
				if (Object.keys(obj)[0] === 'Users') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.users = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.users = 'write';
				}
				if (Object.keys(obj)[0] === 'Groups') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.groups = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.groups = 'write';
				}
				if (Object.keys(obj)[0] === 'Servers') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.servers = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.servers = 'write';
				}
				if (Object.keys(obj)[0] === 'Workers') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.workers = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.workers = 'write';
				}
				if (Object.keys(obj)[0] === 'Eggs') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.eggs = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.eggs = 'write';
				}
				if (Object.keys(obj)[0] === 'SiteSettings') {
					if (obj[Object.keys(obj)[0]] === 'Read') permissions.sitesettings = 'read';
					if (obj[Object.keys(obj)[0]] === 'Write') permissions.sitesettings = 'write';
				}
			}
		}
	}
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
					<Dialog.Root>
						<Dialog.Trigger>
							<Button
								onclick={() => loadPerms(gc[i])}
								disabled={!checkAdvancedPerms(data.layout!.permissions, 'Groups', ['Write'])}
								>Change permissions</Button
							>
						</Dialog.Trigger>
						<Dialog.Content>
							<form action="?/changeperms" method="POST">
								<Dialog.Header>
									<Dialog.Title>Change group perms</Dialog.Title>
									<Dialog.Description
										>To see what permission do what, see the tooltips. Server ownership, and shares
										are not based of groups.</Dialog.Description
									>
								</Dialog.Header>
								<div class="my-4 flex flex-col gap-2">
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label for="login">Login</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>
														If unchecked the user is disabled. The admin privilege overrides this.
													</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Checkbox bind:checked={permissions.login} name="login" id="login" />
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label for="admin">Admin</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>Default admin permission, grants everything.</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Checkbox bind:checked={permissions.admin} name="admin" id="admin" />
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label for="adminpage">Admin Page</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>Grants access to the admin pages.</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Checkbox
											bind:checked={permissions.adminpage}
											name="adminpage"
											id="adminpage"
										/>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>User</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>Read: Can see own settings. Write: Can see and change own settings.</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.user} type="single">
											<Select.Trigger class="w-[180px]">{permissions.user}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Users</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>
														Read: Can see users and their groups. Write: Can see and change users.
													</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.users} type="single">
											<Select.Trigger class="w-[180px]">{permissions.users}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Groups</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>
														Read: Can see groups and their perms. Write: Can see and change groups.
													</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.groups} type="single">
											<Select.Trigger class="w-[180px]">{permissions.groups}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Servers</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>Read: Can see all servers. Write: Can see and modify all servers.</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.servers} type="single">
											<Select.Trigger class="w-[180px]">{permissions.servers}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Workers</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>
														Read: Can see workers and their status. Write: Can see and change
														workers.
													</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.workers} type="single">
											<Select.Trigger class="w-[180px]">{permissions.workers}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Eggs</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>Read: Can see eggs. Write: Can see and change eggs.</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.eggs} type="single">
											<Select.Trigger class="w-[180px]">{permissions.eggs}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
									<div class="flex items-center justify-between">
										<Tooltip.Provider>
											<Tooltip.Root>
												<Tooltip.Trigger>
													<Label>Site settings</Label>
												</Tooltip.Trigger>
												<Tooltip.Content>
													<p>
														Read: Can see site settings. Write: Can see and change site settings.
													</p>
												</Tooltip.Content>
											</Tooltip.Root>
										</Tooltip.Provider>
										<Select.Root bind:value={permissions.sitesettings} type="single">
											<Select.Trigger class="w-[180px]">{permissions.sitesettings}</Select.Trigger>
											<Select.Content>
												{#each modifiers as md}
													<Select.Item value={md}>{md}</Select.Item>
												{/each}
											</Select.Content>
										</Select.Root>
									</div>
								</div>
								<input required type="text" name="name" id="name" hidden bind:value={gc[i]} />
								<input hidden type="text" name="user" value={`user/${permissions.user}`} />
								<input hidden type="text" name="users" value={`users/${permissions.users}`} />
								<input hidden type="text" name="groups" value={`groups/${permissions.groups}`} />
								<input hidden type="text" name="servers" value={`servers/${permissions.servers}`} />
								<input hidden type="text" name="workers" value={`workers/${permissions.workers}`} />
								<input hidden type="text" name="eggs" value={`eggs/${permissions.eggs}`} />
								<input
									hidden
									type="text"
									name="sitesettings"
									value={`sitesettings/${permissions.sitesettings}`}
								/>
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
