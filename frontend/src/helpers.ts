import { state } from "./stores";

export const emailSupport = () => {
    state.subscribe((state) => {
        window.location.href = `
            mailto:${state.data["support_email"]}?subject=${state.app_title} v${state.app_version} Support Request (Error: ${state.error ?? "None or Unknown"})`;
    });
};
