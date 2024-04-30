import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const content = `This is the content for template ${params.tabId}`;

    return {
        content,
    };
};
