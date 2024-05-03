import type { LayoutLoad } from './$types';
import { DEPENDENCY } from '$lib/const';
import { retrieveState } from '$lib/adapters';

export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ depends }) => {
    depends(DEPENDENCY.STATE);
    const state = await retrieveState();

    return {
        state,
    };
};
