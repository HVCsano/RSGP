<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';

	let { data, form } = $props();
</script>

<svelte:head>
	<title>RSGP - Login</title>
</svelte:head>
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
