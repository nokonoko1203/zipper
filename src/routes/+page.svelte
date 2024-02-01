<script>
	import { dialog } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/tauri';

	let inputPaths = [];
	let outputPath = '';

	async function openFileDialog() {
		const res = await dialog.open({
			multiple: true,
			directory: false
		});
		if (!res) return;
		inputPaths = Array.isArray(res) ? res : [res];
	}

	async function openOutputDialog() {
		const res = await dialog.save({
			filters: [
				{
					name: 'Output format',
					extensions: ['zip']
				}
			]
		});
		outputPath = Array.isArray(res) ? res[0] : res;
	}

	async function compressFiles() {
		if (!inputPaths) {
			alert('入力フォルダ/ファイルを選択してください');
			return;
		}
		if (!outputPath) {
			alert('出力先を選択してください');
			return;
		}

		await invoke('run', {
			inputPaths,
			outputPath
		});
		alert('ファイルの圧縮と出力が完了しました。');
	}
</script>

<div class="text-center flex flex-col gap-12 my-12">
	<div class="text-center flex flex-col gap-12 my-12">
		<h1 class="text-2xl font-bold text-red-700">ジップマン</h1>

		<div>
			<button on:click={openFileDialog}>入力ファイル選択</button>
			<button on:click={openOutputDialog}>出力ファイル選択</button>
		</div>
		<div>
			<p>入力ファイル:</p>
			{#if inputPaths.length > 0}
				{#each inputPaths as path}
					<p>{path}</p>
				{/each}
			{/if}
			<p>保存先:</p>
			{#if outputPath}
				<p>{outputPath}</p>
			{/if}
		</div>
		<div>
			<button on:click={compressFiles}>圧縮</button>
		</div>
	</div>
</div>
