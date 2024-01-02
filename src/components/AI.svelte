<script lang='ts'>
    import { onMount } from "svelte";
    import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";
    export let name: undefined | string, path: string;

    onMount(() => {
        async function getItems(){
            let tempPath = path?.replace(/\\/g, "/").split("/").slice(-1).toString();
            let fileGuts = await readTextFile(`networks/${tempPath}`, { dir: BaseDirectory.AppData });

            description = fileGuts.split("description:")[1].split(";")[0];
            date = fileGuts.split("date:")[1].split(";")[0];
        }

        getItems();
    })

    let description: string, date: string;
</script>

<div class="flex flex-col items-start justify-start p-1 mx-2 aspect-[12/5] my-3 rounded-lg bg-midnight-900/45">
    <section>
        <span class="text-3xl font-mono tracking-tight pr-6 border-b-2 leading-6 border-b-neptune-400">{name?.replace('.txt', '')}</span>
        <p>Created on: <span class="text-neptune-200 italic tracking-tight">{date}</span></p>
        <span class="tracking-tight line-clamp-2 lg:line-clamp-1 2xl:line-clamp-2 my-1">{description}</span>
    </section>
    <section class="flex space-x-3">
        <button title="Play" class="text-green-400">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
            </svg>              
        </button>
        <button title="Delete" class="text-red-400">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
            </svg>              
        </button>
        <button title="Share" class="text-neptune-300">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M7.217 10.907a2.25 2.25 0 1 0 0 2.186m0-2.186c.18.324.283.696.283 1.093s-.103.77-.283 1.093m0-2.186 9.566-5.314m-9.566 7.5 9.566 5.314m0 0a2.25 2.25 0 1 0 3.935 2.186 2.25 2.25 0 0 0-3.935-2.186Zm0-12.814a2.25 2.25 0 1 0 3.933-2.185 2.25 2.25 0 0 0-3.933 2.185Z" />
            </svg>              
        </button>
    </section>
</div>