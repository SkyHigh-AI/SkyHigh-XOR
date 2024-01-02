<script lang="ts">
    import Ai from "../components/AI.svelte";
    import { writeTextFile, createDir, readDir, exists, BaseDirectory, type FileEntry } from "@tauri-apps/api/fs";
    import { onMount } from "svelte";

    let networksArray: FileEntry[] = [];
    let loading = true;

    onMount(() => {
        async function newAI(){
            let folderExists = await exists('confirmFile.txt', { dir: BaseDirectory.AppData });
            if(!folderExists){
                await createDir('networks', { dir: BaseDirectory.AppData, recursive: true });
                await writeTextFile('confirmFile.txt',  'This file just confirms that the "networks" folder exists', { dir: BaseDirectory.AppData });
            }

            networksArray = await readDir('networks', { dir: BaseDirectory.AppData, recursive: true });
            loading = false;
        }

        newAI();
    });
</script>

{#if loading}
    <main class="min-h-fullscreen w-screen flex flex-col justify-center items-center px-4">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-32 h-32 animate-spin-slow text-neptune-300">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006 0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3-3 3" />
        </svg>
        <span class="text-4xl font-mono tracking-tight">Loading...</span>          
    </main>
{:else}
    {#if networksArray.length === 0}
    <main class="min-h-fullscreen w-screen flex flex-col justify-center px-4">
        <p class="self-center text-center text-3xl font-mono font-medium tracking-tight">You don't appear to have any AI's yet.<br/>Create one with the "<span class="text-neptune-300 text-4xl drop-shadow-navBtn">+</span>" at the top!</p>
    </main>
    {:else}
    <main class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 px-4 py-3">
        {#each networksArray as network}
            <Ai name={network.name} path={network.path}></Ai>
        {/each}
    </main>
    {/if}
{/if}