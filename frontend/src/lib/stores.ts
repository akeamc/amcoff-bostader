import { writable } from 'svelte/store';
import type { UserDetails } from './api';

export type AuthState =
	| { status: 'loading' }
	| { status: 'authenticated'; user: UserDetails }
	| { status: 'unauthenticated' };

function createAuthStore() {
	const { subscribe, set } = writable<AuthState>({ status: 'loading' });

	return {
		subscribe,
		setUser(user: UserDetails) {
			set({ status: 'authenticated', user });
		},
		setUnauthenticated() {
			set({ status: 'unauthenticated' });
		},
		setLoading() {
			set({ status: 'loading' });
		}
	};
}

export const auth = createAuthStore();

export type SortField =
	| 'queue_position'
	| 'rent'
	| 'size_sqm'
	| 'reserve_until'
	| 'move_in'
	| 'floor';

export const SORT_NAMES: Record<SortField, string> = {
	queue_position: 'Köplats',
	rent: 'Hyra',
	size_sqm: 'Golvyta',
	reserve_until: 'Sista anmälningsdag',
	move_in: 'Inflyttning',
	floor: 'Våning'
};

export const sortField = writable<SortField>('queue_position');
