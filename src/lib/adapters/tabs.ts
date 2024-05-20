import { invoke } from '@tauri-apps/api/tauri';
import { COMMAND } from '$lib/const';

export type ServerTab = {
    id: string;
    title: string;
    mode: SessionMode;
};
export type Tab = {
    link: string;
    title: ServerTab['title'];
    mode: SessionMode;
};
export type SessionMode = 'OneOff' | 'Chat' | 'File';

export const deseralizeTab = (tab: ServerTab): Tab => ({
    link: `/${tab.id}`,
    title: tab.title,
    mode: tab.mode,
});

export const createTab = async (): Promise<Tab> => {
    const newTab: ServerTab = await invoke(COMMAND.CREATE_TAB);
    return deseralizeTab(newTab);
};

export const updateTabTitle = async (tab: Tab): Promise<Tab> => {
    const updated: ServerTab = await invoke(COMMAND.UPDATE_TAB_TITLE, {
        id: tab.link.slice(1),
        title: tab.title,
    });
    return deseralizeTab(updated);
};

export const deleteTab = async (link: string): Promise<Tab> => {
    const deleted: ServerTab = await invoke(COMMAND.DELETE_TAB, { id: link.slice(1) });
    return deseralizeTab(deleted);
};
