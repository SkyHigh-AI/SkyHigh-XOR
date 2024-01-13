<script lang="ts">
    import type { PageData } from "./$types";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    // import { confirm } from "@tauri-apps/api/dialog";
    import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

    onMount(() => {
        async function getItems(){
            let fileGuts = await readTextFile(`networks/${data.ai.path}`, { dir: BaseDirectory.AppData });

            name = data.ai.path.replace(".txt", "");
            description = fileGuts.split("description:")[1].split(";")[0];
            date = fileGuts.split("date:")[1].split(";")[0];
            hiddenNodes = parseInt(fileGuts.split("hn:")[1].split(";")[0]);
            learnRate = parseFloat(fileGuts.split("lr:")[1].split(";")[0]);

            hasTrained = fileGuts.split("hasTrained:")[1].split(";")[0];

            await invoke('loadFromSave', {fileGuts});
            loading = false;
        }

        getItems();
    });

    let name: string, description: string, date: string, hiddenNodes: number, learnRate: number, hasTrained: string, rustWait;
    let loading = true;

	export let data: PageData;
</script>

{#if loading}
    <main class="min-h-fullscreen w-screen flex flex-col justify-center items-center px-4">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-32 h-32 animate-spin-slow text-neptune-300">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006 0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3-3 3" />
        </svg>
        <span class="text-4xl font-mono tracking-tight">Loading...</span>          
    </main>
{:else}
    <main>
        <h1>{name}</h1>
    </main>
{/if}