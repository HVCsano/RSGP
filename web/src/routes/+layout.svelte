<script lang="ts">
	import '../app.css';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';

	import { ModeWatcher } from 'mode-watcher';

	let { children, data } = $props();
</script>

<svelte:head>
	<title>Rust Game Panel</title>
	<meta name="description" content="A simple game panel written in Rust." />
	<meta content="Rust Game Panel" property="og:title" />
	<meta content="/" property="og:url" />
	<meta content="A simple game panel written in Rust." property="og:description" />
	<meta content="/favicon.png" property="og:image" />
	<meta content="#CE422B" data-react-helmet="true" name="theme-color" />
</svelte:head>

<ModeWatcher />

{#if data.noauth}
	<main>
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
				<form>
					<div class="flex flex-col gap-6">
						<div class="grid gap-2">
							<Label for="email">Username</Label>
							<Input id="email" type="text" placeholder="peldabela" required />
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
							<Input id="password" type="password" required />
						</div>
					</div>
				</form>
			</Card.Content>
			<Card.Footer class="flex-col gap-2">
				<Button type="submit" class="w-full">Login</Button>
			</Card.Footer>
		</Card.Root>
	</main>
{:else}
	{@render children()}
{/if}

<footer>
	<h1 class="absolute bottom-2 left-2">
		Made by <a class="font-bold text-[#76ff7a]" target="_blank" href="https://csano.hu">HVCsano</a>,
		with ❤️
	</h1>
</footer>
