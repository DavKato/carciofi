<script lang="ts">
    import { goto } from '$app/navigation';
    import { CloseIcon, PlusIcon } from '$lib/icons';

    let { path } = $props();

    let tabs = $state([
        { link: '/1', name: 'tab1' },
        { link: '/2', name: 'tab2' },
        { link: '/3', name: 'tab3' },
    ]);

    let currentLink = $derived(tabs.find((tab) => tab.link === path)?.link);

    const closeTab = (link: string) => {
        return (e: MouseEvent) => {
            e.preventDefault();
            const index = tabs.findIndex((tab) => tab.link === link);
            const updatedTabs = tabs.filter((tab) => tab.link !== link);
            if (link === currentLink)
                goto(updatedTabs[Math.max(0, index - 1)]?.link, { replaceState: true });
            tabs = updatedTabs;
        };
    };
</script>

<header class="pt-1 grid auto-rows-[1.8rem] text-sm bg-paper-7">
    <nav class="h-full px-3 flex gap-4">
        <button class="h-full">=★</button>
        <button class="back" onclick={() => window.history.back()}>{'<'}</button>
        <button class="next" onclick={() => window.history.forward()}>{'>'}</button>

        <div class="templates w-full flex justify-center gap-4">
            <button>EN -> JP</button>
            <button>JP -> EN</button>
        </div>
    </nav>
    <nav class="flex justify-evenly gap-px bg-paper-9 border-b border-paper-1">
        {#each tabs as tab}
            <a
                class="h-full px-3 flex-grow flex items-center cursor-default {currentLink ===
                tab.link
                    ? 'bg-paper-7'
                    : 'bg-paper-3 transition-colors hover:bg-paper-6 hover:duration-200'}"
                href={tab.link}
            >
                {#if tabs.length > 1}
                    <CloseIcon
                        role="button"
                        class="h-5 w-5 p-0.5 aspect-square rounded-sm cursor-default transition-colors hover:bg-paper-10"
                        onclick={closeTab(tab.link)}
                    />
                {/if}
                <span class="flex-grow text-center">{tab.name}</span>
            </a>
        {/each}
        <PlusIcon
            role="button"
            class="h-full w-8 px-2 bg-paper-3 hover:bg-paper-7 transition-colors duration-200"
        />
    </nav>
</header>

<style lang="postcss">
</style>
