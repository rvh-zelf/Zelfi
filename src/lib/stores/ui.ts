import { writable } from 'svelte/store';

export type View = 'goals' | 'inbox' | 'history' | 'memory' | 'settings';

export const activeView = writable<View>('goals');

// LM Studio connection state — shared across all views
export const lmBaseUrl = writable<string>(
  typeof localStorage !== 'undefined'
    ? (localStorage.getItem('zelfi_lm_url') ?? 'http://127.0.0.1:1234')
    : 'http://127.0.0.1:1234'
);

export const lmConnected = writable<boolean>(false);

// Inbox unread count — drives the tray badge (future)
export const inboxUnread = writable<number>(0);
