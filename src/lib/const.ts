export const COMMAND = {
    RETRIEVE_STATE: 'retrieve_state',
    GET_TAB_HEADER: 'get_tab_header',
    CREATE_TAB: 'create_tab',
    UPDATE_TAB_TITLE: 'update_tab_title',
    DELETE_TAB: 'delete_tab',
    GET_CONTENT: 'get_content',
};

export const DEPENDENCY: Record<string, `${string}:${string}`> = {
    STATE: 'app:state',
};
