<script>
    import '../app.css';
    import Header from './Header.svelte';
    import Footer from './Footer.svelte';

    let { data, children } = $props();
</script>

<div class="grid dark:bg-stone-900 dark:text-stone-200">
    {#key data.state}
        <Header tabs={data.state.tabs}></Header>
    {/key}

    <main>
        {@render children()}
    </main>

    <Footer></Footer>
</div>

<style lang="postcss">
    :global(html) {
        font-family: 'Arial', sans-serif;
    }
    :global(body) {
        @apply m-0 p-0 bg-paper-3 text-ink-1;
    }

    .grid {
        grid-template-rows: auto 1fr auto;
        height: 100vh;
    }
</style>
