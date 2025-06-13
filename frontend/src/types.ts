export interface AppState {
    app_title: string;
    app_version: string;
    data: Record<string, any>;
    debug: boolean;
    error: string | null;
    loading: boolean;
}
