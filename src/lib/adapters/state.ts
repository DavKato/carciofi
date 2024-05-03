import { invoke } from '@tauri-apps/api/tauri';
import { COMMAND } from '$lib/const';
import { deseralizeTab, type Tab, type ServerTab } from './tabs';

export const retrieveState = async () => {
    const state: SavedState = await invoke(COMMAND.RETRIEVE_STATE);
    return deserializeState(state);
};

const deserializeState = (saved: SavedState): State => {
    return {
        tabs: saved.tabs.map(deseralizeTab),
    };
};

export type SavedState = {
    tabs: ServerTab[];
};
export type State = {
    tabs: Tab[];
};
