<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';

	let { data } = $props();
	let id = $state(data.info.id);
</script>

<form action="?/startserver" method="POST">
	<input type="text" bind:value={id} name="id" hidden />
	<Button variant="outline" type="submit" disabled={data.info.state == 'Running'}
		>Start server</Button
	>
</form>
<form action="?/stopserver" method="POST">
	<input type="text" bind:value={id} name="id" hidden />
	<Button variant="destructive" type="submit" disabled={data.info.state == 'Stopped'}
		>Stop server</Button
	>
</form>

<div class="mx-auto w-[60dvw] text-center">
	<h2>Log</h2>
	<pre class="text-left">
		{data.log}
	</pre>
</div>
