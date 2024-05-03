<script lang="ts">
    import type { Tab } from '$lib/adapters';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { onDestroy, tick } from 'svelte';
    import { revalidateState } from '$lib/actions';
    import { createTab, updateTabTitle, deleteTab } from '$lib/adapters';
    import { CloseIcon, PlusIcon, EditPenIcon } from '$lib/icons';

    type Prop = {
        tabs: Tab[];
    };
    let { tabs }: Prop = $props();

    let path = $state('');
    const unsub = page.subscribe((value) => {
        path = value.url.pathname;
    });

    let tabIndexBeingRenamed: number | undefined = $state();
    let input: HTMLInputElement | undefined = $state();

    const addTab = async () => {
        const newTab = await createTab();
        await revalidateState();
        await goto(newTab.link);
    };

    const closeTab = (tab: Tab) => {
        return async (e: MouseEvent) => {
            e.preventDefault();
            const index = tabs.findIndex((t) => t.link === tab.link);
            await deleteTab(tab.link);
            await revalidateState();
            if (tab.link === path)
                await goto(tabs[Math.max(0, index - 1)]?.link, { replaceState: true });
        };
    };

    const editTabTitle = (index: number) => {
        return async (e: MouseEvent) => {
            e.preventDefault();
            tabIndexBeingRenamed = index;
            await tick();
            input?.select();
        };
    };

    const confirmTabTitle = async (e: FocusEvent | KeyboardEvent) => {
        const input = e.target as HTMLInputElement;
        const newTitle = input.value;
        await updateTabTitle({ link: tabs[tabIndexBeingRenamed!].link, title: newTitle });
        await revalidateState();
        tabIndexBeingRenamed = undefined;
    };

    onDestroy(() => {
        unsub();
    });
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
    <nav class="flex bg-paper-9 border-b border-paper-1">
        <div
            class="flex-grow inline-grid grid-cols-[repeat(auto-fit,minmax(0,1fr))] justify-evenly gap-px"
        >
            {#each tabs as tab, i}
                <a
                    class="group h-full px-3 grid grid-cols-[auto,1fr,auto] items-center cursor-default {path ===
                    tab.link
                        ? 'bg-paper-7'
                        : 'bg-paper-3 transition-colors hover:bg-paper-6 hover:duration-200'}"
                    href={tab.link}
                    ondblclick={editTabTitle(i)}
                >
                    <div>
                        {#if tabs.length > 1}
                            <CloseIcon
                                role="button"
                                class="opacity-0 h-5 w-5 p-0.5 rounded-sm cursor-default transition group-hover:opacity-100 hover:bg-paper-10 duration-200"
                                onclick={closeTab(tab)}
                            />
                        {/if}
                    </div>

                    {#if tabIndexBeingRenamed === i}
                        <input
                            bind:this={input}
                            type="text"
                            class="text-center p-0 border-none bg-transparent text-inherit focus:ring-paper-10 ring-offset-ink-1"
                            value={tab.title}
                            onblur={confirmTabTitle}
                            onkeydown={(e) => {
                                e.key === 'Enter' && confirmTabTitle(e);
                                e.key === 'Escape' && (tabIndexBeingRenamed = undefined);
                            }}
                        />
                    {:else}
                        <span class="text-center">{tab.title}</span>
                    {/if}

                    <EditPenIcon
                        role="button"
                        class="opacity-0 h-5 w-5 p-0.5 rounded-sm transition group-hover:opacity-100 hover:bg-paper-10 duration-200"
                        onclick={editTabTitle(i)}
                    ></EditPenIcon>
                </a>
            {/each}
        </div>
        <PlusIcon
            role="button"
            class="h-full w-8 px-2 bg-paper-3 hover:bg-paper-7 transition-colors duration-200"
            onclick={addTab}
        />
    </nav>
</header>

<style lang="postcss">
</style>
