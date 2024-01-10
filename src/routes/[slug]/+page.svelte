<script lang="ts">
    import type { PageData } from "./$types";
    import { onMount } from "svelte";
    // import { invoke } from "@tauri-apps/api/tauri";
    // import { confirm } from "@tauri-apps/api/dialog";
    import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

    onMount(() => {
        async function getItems(){
            let fileGuts = await readTextFile(`networks/${data.ai.path}`, { dir: BaseDirectory.AppData });

            name = data.ai.path.replace(".txt", "");
            description = fileGuts.split("description:")[1].split(";")[0];
            date = fileGuts.split("date:")[1].split(";")[0];
            hiddenNodes = parseInt(fileGuts.split("hiddenNodes:")[1].split(";")[0]);
            learnRate = parseFloat(fileGuts.split("learnRate:")[1].split(";")[0]);

            hasTrained = fileGuts.split("hasTrained:")[1].split(";")[0];

            let tempArary: number[] = [];
            for(let i = 0; i < hiddenNodes; i++){
                let tempNum = fileGuts.split(`hN${i}:`)[1].split(";")[0];
                tempArary.push(parseFloat(tempNum));
            }
            hNArray = tempArary;
        }

        getItems();
    });

    let name: string, description: string, date: string, hiddenNodes: number, learnRate: number, hasTrained: string, hNArray: number[] = [];

	export let data: PageData;
</script>