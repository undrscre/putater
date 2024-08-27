<script lang="ts">
    import Main from "./lib/view/main.svelte";
    // import Documentation from "./lib/view/documentation.svelte";
    import Error from "./lib/view/error.svelte"
    
    let view: string = "main";
</script>

<main>
    {#if view === "main"} 
        <Main />
    {:else}
        <Error />
    {/if}
</main>