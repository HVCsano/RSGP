<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import { checkAdvancedPerms } from '$lib/api';
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
	{#each data.users as user}
		<Card.Root>
			<Card.Header>
				<Card.Title>{user.username}</Card.Title>
			</Card.Header>
			<Card.Content>
				<h1 class="text-lg font-bold">Permissions:</h1>
				{#each user.permissions as perm}
					{#if typeof perm === 'string'}
						<p>{perm}</p>
					{:else}
						<p>{Object.keys(perm)[0]}: {perm[Object.keys(perm)[0]]}</p>
					{/if}
				{/each}
			</Card.Content>
			<Card.Footer>
				<div class="flex flex-col gap-2">
					<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
						>Change password</Button
					>
					<Button disabled={!checkAdvancedPerms(data.layout!.permissions, 'Users', ['Write'])}
						>Change permissions</Button
					>
				</div>
			</Card.Footer>
		</Card.Root>
	{/each}
</div>
