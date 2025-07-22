<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { checkAdvancedPerms } from '$lib/api';

	let ses_clone = $state(data.sessions);
</script>

<svelte:head>
	<title>RSGP - User Settings</title>
</svelte:head>

<h1 class="text-center">TODO: Password change</h1>

<div class="mx-auto flex w-full flex-wrap items-center justify-center px-8">
	<Card.Root class="w-full">
		<Card.Header>
			<h1>Logged in sessions</h1>
		</Card.Header>
		<Table.Root class="text-center">
			<Table.Header>
				<Table.Row>
					<Table.Head class="text-center">Name</Table.Head>
					<Table.Head class="text-center">Login time</Table.Head>
					<Table.Head class="text-center">Expiration time</Table.Head>
					<Table.Head class="text-center">Manage</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each data.sessions as s, i}
					<Table.Row>
						<Table.Cell
							><ScrollArea orientation="horizontal" class="mx-auto w-full md:w-[20dvw]"
								>{s.agent}</ScrollArea
							></Table.Cell
						>
						<Table.Cell
							>{new Date(s.login_time * 1000).getFullYear()}. {new Date(
								s.login_time * 1000
							).getMonth()}. {new Date(s.login_time * 1000).getDate()}. {new Date(
								s.login_time * 1000
							).getHours()}:{new Date(s.login_time * 1000).getMinutes()}:{new Date(
								s.login_time * 1000
							).getSeconds()}</Table.Cell
						>
						<Table.Cell
							>{new Date(s.exp_time * 1000).getFullYear()}. {new Date(
								s.exp_time * 1000
							).getMonth()}. {new Date(s.exp_time * 1000).getDate()}. {new Date(
								s.exp_time * 1000
							).getHours()}:{new Date(s.exp_time * 1000).getMinutes()}:{new Date(
								s.exp_time * 1000
							).getSeconds()}</Table.Cell
						>
						<Table.Cell class="flex items-center justify-center gap-2">
							<Dialog.Root>
								<Dialog.Trigger>
									<Button
										type="submit"
										disabled={!checkAdvancedPerms(data.layout!.permissions, 'User', ['Write'])}
										>Change name</Button
									>
								</Dialog.Trigger>
								<Dialog.Content>
									<form action="?/changename" method="POST">
										<Dialog.Header>
											<Dialog.Title>Change name dialog</Dialog.Title>
											<Dialog.Description>
												<Label for="name">Enter new name</Label>
												<Input required type="text" name="name" id="name" />
											</Dialog.Description>
										</Dialog.Header>
										<Dialog.Footer class="mt-2">
											<input type="text" name="id" bind:value={ses_clone[i].id} hidden />
											<Button type="submit">Change name</Button>
										</Dialog.Footer>
									</form>
								</Dialog.Content>
							</Dialog.Root>
							<AlertDialog.Root>
								<AlertDialog.Trigger
									><Button
										variant="destructive"
										disabled={!checkAdvancedPerms(data.layout!.permissions, 'User', ['Write'])}
										>Log out</Button
									></AlertDialog.Trigger
								>
								<AlertDialog.Content>
									<AlertDialog.Header>
										<AlertDialog.Title>Are you sure?</AlertDialog.Title>
										<AlertDialog.Description>
											This action will log out this session.
										</AlertDialog.Description>
									</AlertDialog.Header>
									<AlertDialog.Footer>
										<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
										<form action="?/remove" method="POST">
											<input type="text" name="id" bind:value={ses_clone[i].id} hidden />
											<AlertDialog.Action>Continue</AlertDialog.Action>
										</form>
									</AlertDialog.Footer>
								</AlertDialog.Content>
							</AlertDialog.Root>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
		<Card.Footer>
			<AlertDialog.Root>
				<AlertDialog.Trigger
					><Button
						variant="destructive"
						disabled={!checkAdvancedPerms(data.layout!.permissions, 'User', ['Write'])}
						>Log out everywhere</Button
					></AlertDialog.Trigger
				>
				<AlertDialog.Content>
					<AlertDialog.Header>
						<AlertDialog.Title>Are you sure?</AlertDialog.Title>
						<AlertDialog.Description>
							This action will log out all sessions.
						</AlertDialog.Description>
					</AlertDialog.Header>
					<AlertDialog.Footer>
						<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
						<form action="?/remove_all" method="POST">
							<AlertDialog.Action>Continue</AlertDialog.Action>
						</form>
					</AlertDialog.Footer>
				</AlertDialog.Content>
			</AlertDialog.Root>
		</Card.Footer>
	</Card.Root>
</div>
