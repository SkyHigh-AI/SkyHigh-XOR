<script lang="ts">
    import type { PageData } from "./$types";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { message } from "@tauri-apps/api/dialog";
    import { readTextFile, writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";

    onMount(() => {
        async function getItems(){
            let fileGuts = await readTextFile(`networks/${data.ai.path}`, { dir: BaseDirectory.AppData });

            name = data.ai.path.replace(".txt", "");
            description = fileGuts.split("description:")[1].split(";")[0];
            date = fileGuts.split("date:")[1].split(";")[0];
            hiddenNodes = parseInt(fileGuts.split("hn:")[1].split(";")[0]);
            learnRate = parseFloat(fileGuts.split("lr:")[1].split(";")[0]);

            let _hasTrained = fileGuts.split("hasTrained:")[1].split(";")[0];
            if(_hasTrained === "true") hasTrained = true;
            else if(_hasTrained === "false") hasTrained = false;

            await invoke('loadFromSave', {fileGuts});
            isLoading = false;
        }

        getItems();
    });

    let name: string, description: string, date: string, hiddenNodes: number, learnRate: number, hasTrained: boolean;
    let isLoading = true, isTraining = false;
    let dialog: HTMLDialogElement, input1Val: string | undefined, input2Val: string | undefined;
    let responses: string[] = [];

    async function trainAI(){
        isTraining = true;
        let output = await invoke('trainNetwork');

        await writeTextFile(`networks/${data.ai.path}`, `description:${description};lr:${learnRate};hn:${hiddenNodes};date:${date};hasTrained:true;${output}`, { dir: BaseDirectory.AppData });
        isTraining = false;
        hasTrained = true;
        window.location.reload();
    }

    async function checkSubmit(){
        if(!input1Val || !input2Val){
            await message("Some fields have not been filed out.\nPlease fill all fields before clicking sumbit :)", { title: "SkyHigh AI", type: "error"});
            return;
        }
        if((input1Val.toUpperCase() != "TRUE" && input1Val.toUpperCase() != "FALSE") || (input2Val.toUpperCase() != "TRUE" && input2Val.toUpperCase() != "FALSE")){
            await message("Some inputs do not equal the words true or false.\nPlease make sure the only words you enter are true or false :)", { title: "SkyHigh AI", type: "error"});
            return;
        }

        let guess: number[] = await invoke('networkGuess', {input1: input1Val.toUpperCase(), input2: input2Val.toUpperCase()});
        console.log(guess);

        let tempArray = responses;
        if(guess[0] > guess[1]) tempArray.push("True!");
        else tempArray.push("False!");
        responses = tempArray;

        dialog.close();
        input1Val = undefined;
        input2Val = undefined;
    }

	export let data: PageData;
</script>

<dialog bind:this={dialog} class="bg-midnight-900 backdrop:backdrop-blur-[2px] sm:w-2/3 lg:w-1/2 2xl:w-1/3 text-cotton rounded">
    <section class="flex justify-between border-b border-cotton/65 px-2 py-1">
        <span class="text-4xl font-mono tracking-tight font-semibold">Use AI</span>
        <button on:click={() => dialog.close()} class="hover:text-red-400 hover:drop-shadow-redBtn transition-all duration-200 ease-out">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
            </svg>
        </button>
    </section>
    <section class="flex flex-col items-center space-y-4 w-full px-2 py-1">
        <section class="flex w-full space-x-4 px-6">
            <input type="text" bind:value={input1Val} name="Input 1" autocomplete="off" class="rounded-md bg-midnight-700 outline-0 h-8 pl-2 w-full placeholder:italic" placeholder="Input 1 (true/false)">
            <input type="text" bind:value={input2Val} name="Input 2" autocomplete="off" class="rounded-md bg-midnight-700 outline-0 h-8 pl-2 w-full placeholder:italic" placeholder="Input 2 (true/false)">
        </section>
        <button on:click={() => checkSubmit()} class="flex items-center mx-2 my-1 space-x-2 px-1.5 py-0.5 rounded-md hover:drop-shadow-greenBtn hover:text-green-400 transition-all duration-200 ease-out">
            <span class="text-2xl">Submit</span>
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 12 3.269 3.125A59.769 59.769 0 0 1 21.485 12 59.768 59.768 0 0 1 3.27 20.875L5.999 12Zm0 0h7.5" />
            </svg>              
        </button>
    </section>
</dialog>

{#if isLoading}
    <main class="min-h-fullscreen w-screen flex flex-col justify-center items-center px-4">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-32 h-32 animate-spin-slow text-neptune-300">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006 0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3-3 3" />
        </svg>
        <span class="text-4xl font-mono tracking-tight">Loading...</span>
    </main>
{:else}
    <main class="relative">
        <div class="flex w-fit px-4">
            <section class="flex border-b-2 border-b-neptune-300 pr-4 w-fit py-2">
                <h1 class="text-6xl font-mono tracking-tight font-bold self-center">{name}</h1>
                <p class="h-fit self-end">Created on: <span class="text-neptune-400">{date}</span></p>
            </section>
            <section class="flex items-center space-x-4">
                <button on:click={() => trainAI()} title="Train" class="py-1 hover:text-neptune-200 hover:drop-shadow-aiBtn transition-all duration-150 ease-out">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.25" stroke="currentColor" class="w-12 h-12">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.26 10.147a60.438 60.438 0 0 0-.491 6.347A48.62 48.62 0 0 1 12 20.904a48.62 48.62 0 0 1 8.232-4.41 60.46 60.46 0 0 0-.491-6.347m-15.482 0a50.636 50.636 0 0 0-2.658-.813A59.906 59.906 0 0 1 12 3.493a59.903 59.903 0 0 1 10.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.717 50.717 0 0 1 12 13.489a50.702 50.702 0 0 1 7.74-3.342M6.75 15a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm0 0v-3.675A55.378 55.378 0 0 1 12 8.443m-7.007 11.55A5.981 5.981 0 0 0 6.75 15.75v-1.5" />
                    </svg>
                </button>
                <button on:click={() => dialog.showModal()} title="Use" disabled={!hasTrained} class="py-1 disabled:text-cotton/25 disabled:hover:cursor-not-allowed enabled:hover:text-neptune-200 enabled:hover:drop-shadow-aiBtn transition-all duration-150 ease-out">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.25" stroke="currentColor" class="w-12 h-12">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z" />
                    </svg>
                </button>
                <button title="Settings" class="py-1 hover:text-neptune-200 hover:drop-shadow-aiBtn transition-all duration-150 ease-out">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.25" stroke="currentColor" class="w-12 h-12">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.43l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.431l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.247a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.086.22-.128.332-.183.582-.495.644-.869l.214-1.28Z" />
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
                    </svg>
                </button>
            </section>
        </div>
        <div class="w-1/2 md:w-2/3 h-aiPage flex flex-col items-center justify-center">
            {#if isTraining}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-32 h-32 animate-spin-slow text-neptune-300">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 12c0-1.232-.046-2.453-.138-3.662a4.006 4.006 0 0 0-3.7-3.7 48.678 48.678 0 0 0-7.324 0 4.006 4.006 0 0 0-3.7 3.7c-.017.22-.032.441-.046.662M19.5 12l3-3m-3 3-3-3m-12 3c0 1.232.046 2.453.138 3.662a4.006 4.006 0 0 0 3.7 3.7 48.656 48.656 0 0 0 7.324 0 4.006 4.006 0 0 0 3.7-3.7c.017-.22.032-.441.046-.662M4.5 12l3 3m-3-3-3 3" />
                </svg>
                <span class="text-4xl font-mono tracking-tight">Loading...</span>
            {/if}
        </div>
        <aside class="border-l border-cotton/65 w-1/2 md:w-1/3 absolute right-0 top-0 h-fullscreen flex flex-col">
            <section class="border-b-2 border-b-neptune-300 flex items-center">
                <h2 class="xl:text-5xl lg:text-4xl text-3xl font-mono tracking-tight font-medium py-2 px-1">Response Window</h2>
                <!-- <button class="hover:text-red-400 hover:drop-shadow-redBtn ml-1 h-fit transition-all duration-200 ease-out">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                    </svg>
                </button> -->
            </section>
            {#if !hasTrained}
                <div class="flex w-full h-full items-center justify-center">
                    <p class="self-center px-1 text-center text-xl font-mono font-medium tracking-tight">You can't use this AI until you train it.<br/>Go to the top and click the graduation cap to get started!</p>
                </div>
            {/if}
            <div class="border h-full">
                {#each responses as item}
                    <span>{item}</span>
                {/each}
            </div>
        </aside>
    </main>
{/if}