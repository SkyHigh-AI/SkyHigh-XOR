<script lang="ts">
    import Ai from "../components/AI.svelte";
    import { onMount } from "svelte";

    let anyNetworks: boolean = false;
    const netowrkArray: any = [];

    onMount(() => {
        anyNetworks = netowrkArray.length != 0;
    });
</script>

{#if !anyNetworks}
<main class="min-h-fullscreen w-screen flex flex-col justify-center px-4">
    <p class="self-center text-center text-3xl font-mono font-medium tracking-tight">You don't appear to have any AI's yet.<br/>Create one with the "<span class="text-neptune-300 text-4xl drop-shadow-navBtn">+</span>" at the top!</p>
</main>
{:else}
<main class="grid md:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-6 px-4 py-3">
    {#each netowrkArray as ai}
        <Ai></Ai>
    {/each}
</main>
{/if}