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

            rustWait = await invoke('loadFromSave', {fileGuts});
            console.log(rustWait);
        }

        getItems();
    });

    let name: string, description: string, date: string, hiddenNodes: number, learnRate: number, hasTrained: string, rustWait;

	export let data: PageData;
</script>