<script>
	import { dialog } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/tauri';

	let inputPaths = [];
	let outputPath = '';
	let formData = {
		name: '',
		description: ''
	};

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

	function updateFormData(event) {
		const { name, value } = event.target;
		formData[name] = value;
	}

	async function compressFiles() {
		if (!inputPaths) {
			alert('入力ファイルを選択してください');
			return;
		}
		if (!outputPath) {
			alert('出力先を選択してください');
			return;
		}

		await invoke('run', {
			inputPaths,
			outputPath,
			formData
		});
		alert('ファイルの圧縮と出力が完了しました。');
	}
</script>

<div class="text-center flex flex-col gap-12 my-12">
	<div class="text-center flex flex-col gap-12 my-12">
		<h1 class="text-2xl font-bold text-red-700">ジップマン</h1>

		<div>
			<button type="button" class="ring-1" on:click={openFileDialog}>入力ファイル選択</button>
			<button type="button" class="ring-1" on:click={openOutputDialog}>出力ファイル選択</button>
		</div>

		<div>
			<p>★入力ファイル</p>
			{#if inputPaths.length > 0}
				{#each inputPaths as path}
					<p>{path}</p>
				{/each}
			{/if}
			<p>★保存先</p>
			{#if outputPath}
				<p>{outputPath}</p>
			{/if}
		</div>
	</div>

	<div>
		<form on:submit|preventDefault={compressFiles} class="flex flex-col">
			<div class="font-bold py-1.5">
				time:
				<input type="text" class="ring-1" id="time" name="time" on:input={updateFormData} />
			</div>
			<div class="font-bold py-1.5">
				platform:
				<input type="text" class="ring-1" id="platform" name="platform" on:input={updateFormData} />
			</div>
			<div class="font-bold py-1.5">
				type:
				<input type="text" class="ring-1" id="type" name="type" on:input={updateFormData} />
			</div>
			<div class="font-bold py-1.5">
				collection:
				<input
					type="text"
					class="ring-1"
					id="collection"
					name="collection"
					on:input={updateFormData}
				/>
			</div>
			<div class="font-bold py-1.5">
				gsd:
				<input type="text" class="ring-1" id="gsd" name="gsd" on:input={updateFormData} />
			</div>
			<div class="font-bold py-1.5">
				assets:
				<input type="text" class="ring-1" id="assets" name="assets" on:input={updateFormData} />
			</div>
			<div class="font-bold py-1.5">
				collection-description:
				<input
					type="text"
					class="ring-1"
					id="collection-description"
					name="collection-description"
					on:input={updateFormData}
				/>
			</div>
			<div>
				<button class="ring-1" type="submit">圧縮</button>
			</div>
		</form>
	</div>
</div>
