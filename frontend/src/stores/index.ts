import { writable } from "svelte/store";
import { type AppState } from "../types";

export const formData = writable<Object>({});
export const serialNumbers = writable<string[]>([]);

export const createAppState = () => {
    const { subscribe, set, update } = writable<AppState>({
        app_title: "[app_title]",
        app_version: "[app_version]",
        data: {},
        debug: false,
        error: null,
        loading: false,
    });

    return {
        create: () => {},
        set,
        setError: (value: string) => update((state) => ({ ...state, value })),
        subscribe,
        update,
    };
};

export const state = createAppState();
