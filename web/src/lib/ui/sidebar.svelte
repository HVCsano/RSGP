<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';

	// User icons
	import HouseIcon from '@lucide/svelte/icons/house';
	import SettingsIcon from '@lucide/svelte/icons/settings';
	import LogoutIcon from '@lucide/svelte/icons/log-out';

	// Admin icons
	import DashboardIcon from '@lucide/svelte/icons/layout-dashboard';
	import ServerIcon from '@lucide/svelte/icons/server';
	import UsersIcon from '@lucide/svelte/icons/users';
	import GroupsIcon from '@lucide/svelte/icons/book-user';
	import WorkersIcon from '@lucide/svelte/icons/pickaxe';
	import EggIcon from '@lucide/svelte/icons/egg';
	import { checkAdvancedPerms, type Permissions } from '$lib/api';

	let props: { admin: boolean; permissions: Permissions[] } = $props();
</script>

<Sidebar.Root>
	<Sidebar.Header class="items-center justify-center">
		<img src="/favicon.png" width="48px" alt="" class="pointer-events-none select-none" />
		<h1 class="textcenter font-bold">Rust Game Panel</h1>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>User</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
							{#snippet child({ props })}
								<a href="/" {...props}>
									<HouseIcon />
									<span>Home</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
							{#snippet child({ props })}
								<a href="/settings" {...props}>
									<SettingsIcon />
									<span>Settings</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
					<Sidebar.MenuItem>
						<Sidebar.MenuButton>
							{#snippet child({ props })}
								<a href="/logout" {...props}>
									<LogoutIcon />
									<span>Logout</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		{#if props.admin}
			<Sidebar.Group>
				<Sidebar.GroupLabel>Admin</Sidebar.GroupLabel>
				<Sidebar.GroupContent>
					<Sidebar.Menu>
						<Sidebar.MenuItem>
							<Sidebar.MenuButton>
								{#snippet child({ props })}
									<a href="/admin" {...props}>
										<DashboardIcon />
										<span>Dashboard</span>
									</a>
								{/snippet}
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
						{#if checkAdvancedPerms(props.permissions, 'Users', ['Write', 'Read'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/users" {...props}>
											<UsersIcon />
											<span>Users</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
						{#if checkAdvancedPerms(props.permissions, 'Groups', ['Write', 'Read'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/groups" {...props}>
											<GroupsIcon />
											<span>Groups</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
						{#if checkAdvancedPerms(props.permissions, 'Servers', ['Read', 'Write'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/servers" {...props}>
											<ServerIcon />
											<span>Servers</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
						{#if checkAdvancedPerms(props.permissions, 'Workers', ['Read', 'Write'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/workers" {...props}>
											<WorkersIcon />
											<span>Workers</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
						{#if checkAdvancedPerms(props.permissions, 'Eggs', ['Read', 'Write'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/eggs" {...props}>
											<EggIcon />
											<span>Eggs</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
						{#if checkAdvancedPerms(props.permissions, 'SiteSettings', ['Read', 'Write'])}
							<Sidebar.MenuItem>
								<Sidebar.MenuButton>
									{#snippet child({ props })}
										<a href="/admin/settings" {...props}>
											<SettingsIcon />
											<span>Site Settings</span>
										</a>
									{/snippet}
								</Sidebar.MenuButton>
							</Sidebar.MenuItem>
						{/if}
					</Sidebar.Menu>
				</Sidebar.GroupContent>
			</Sidebar.Group>
		{/if}
	</Sidebar.Content>
</Sidebar.Root>
