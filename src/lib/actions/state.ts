import { invalidate } from '$app/navigation';
import { DEPENDENCY } from '$lib/const';

export function revalidateState(): Promise<void> {
    return invalidate(DEPENDENCY.STATE);
}
