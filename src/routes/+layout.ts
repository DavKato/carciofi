import type { LayoutLoad } from './$types';
import { getVersion } from '@tauri-apps/api/app';
export const prerender = true;
export const ssr = false;

export const load: LayoutLoad = async ({ url }) => {
    const version = await getVersion();

    return {
        path: url.pathname,
        version,
    };
};
