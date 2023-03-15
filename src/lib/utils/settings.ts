import { Store } from 'tauri-plugin-store-api';
const store = new Store('.settings.dat');

export const get = async <T>(key: string): Promise<T | null> => {
    return store.get(key);
}

export const set = async (key: string, value: unknown): Promise<void> => {
    await store.set(key, value);
    await store.save();
}