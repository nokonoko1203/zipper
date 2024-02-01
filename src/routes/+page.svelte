<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let msg = '';

	async function hello() {
		msg = await invoke('hello', { name });
	}
</script>

<div class="text-center flex flex-col gap-12 my-12">
	<h1 class="text-2xl font-bold text-red-700">Hello MIERUNE</h1>

	<div>
		<input class="border-4" id="hello-input" placeholder="Enter a name..." bind:value={name} />
		<button on:click={hello}>Greet</button>
		<p>{msg}</p>
	</div>
</div>
