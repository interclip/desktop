import { createStore, type Store } from '@tauri-apps/plugin-store';

let store: Store;

const initStore = async () => {
	store = await createStore('.settings.dat', {
		autoSave: 1 as any as boolean
	});
};

initStore();

export const get = async <T>(key: string): Promise<T | null> => {
	if (!store) {
		await initStore();
	}
	return store.get(key);
};

export const set = async (key: string, value: unknown): Promise<void> => {
	if (!store) {
		await initStore();
	}
	await store.set(key, value);
	await store.save();
};
