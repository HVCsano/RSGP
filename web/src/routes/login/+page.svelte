<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';

	import CheckCircle2Icon from '@lucide/svelte/icons/check-circle-2';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';

	import * as Alert from '$lib/components/ui/alert/index.js';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';

	let { form } = $props();
	if (form?.success && browser) {
		setTimeout(() => {
			goto('/', {
				replaceState: true
			});
		}, 300);
	}
	let agent = $state(browser ? window.navigator.userAgent : 'server');
</script>

<svelte:head>
	<title>RSGP - Login</title>
</svelte:head>
<main>
	{#if form?.success}
		<div class="animate-in fade-in mx-auto mt-4 w-[400px] duration-100">
			<Alert.Root>
				<CheckCircle2Icon />
				<Alert.Title>Successful login!</Alert.Title>
				<Alert.Description>You'll be redirected soon!</Alert.Description>
			</Alert.Root>
		</div>
	{/if}
	{#if form?.error}
		<div class="animate-in fade-in mx-auto mt-4 w-[400px] duration-100">
			<Alert.Root variant="destructive">
				<AlertCircleIcon />
				<Alert.Title>Login failed</Alert.Title>
				{#if form.error.startsWith('unknown')}
					<Alert.Description>Error: {form.error.split('unknown/')[1]}</Alert.Description>
				{/if}
				{#if form.error === '401'}
					<Alert.Description>Invalid credentials.</Alert.Description>
				{/if}
				{#if form.error === '406'}
					<Alert.Description>Account is disabled.</Alert.Description>
				{/if}
			</Alert.Root>
		</div>
	{/if}

	<form action="?/login" method="POST">
		<Card.Root class="-translate-1/2 absolute left-1/2 top-1/2 w-[95dvw] md:w-[40dvw] xl:w-[30dvw]">
			<Card.Header>
				<img
					src="/favicon.png"
					width="200px"
					alt=""
					class="pointer-events-none mx-auto select-none"
				/>
				<h1 class="mb-2 text-center text-xl font-bold">Rust Game Panel</h1>
				<Card.Title>Login to your account</Card.Title>
				<Card.Description>Enter your username below to login to your account</Card.Description>
			</Card.Header>
			<Card.Content>
				<div class="flex flex-col gap-6">
					<div class="grid gap-2">
						<Label for="username">Username</Label>
						<Input name="username" type="text" placeholder="peldabela" required />
					</div>
					<div class="grid gap-2">
						<div class="flex items-center">
							<Label for="password">Password</Label>
							<Tooltip.Provider>
								<Tooltip.Root>
									<Tooltip.Trigger
										class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
										>Forgot your password?</Tooltip.Trigger
									>
									<Tooltip.Content>
										<p>The administrators can reset your password.</p>
									</Tooltip.Content>
								</Tooltip.Root>
							</Tooltip.Provider>
						</div>
						<Input name="password" type="password" required />
						<input class="hidden" name="agent" type="text" bind:value={agent} />
					</div>
				</div>
			</Card.Content>
			<Card.Footer class="flex-col gap-2">
				<Button type="submit" class="w-full">Login</Button>
			</Card.Footer>
		</Card.Root>
	</form>
</main>
