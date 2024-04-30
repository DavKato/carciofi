export function once(fn: (event: Event) => void): (event: Event) => void {
    return function (this: unknown, event) {
        if (typeof fn === 'function') fn.call(this, event);
        // @ts-expect-error - this is a one-time event handler
        fn = null;
    };
}

export function preventDefault(fn: (event: Event) => void): (event: Event) => void {
    return function (this: unknown, event) {
        event.preventDefault();
        fn.call(this, event);
    };
}
