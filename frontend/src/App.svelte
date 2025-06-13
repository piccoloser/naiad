<script lang="ts">
    import { onMount } from "svelte";
    import { state } from "./stores";
    import { Router } from "@roxi/routify";
    import { routes } from "../.routify/routes.default";
    import DevPanel from "./lib/DevPanel.svelte";

    const NETWORK_ERROR_MESSAGE = "Network error. Please try again later.";
    const STATE_FETCH_FAIL_MESSAGE = "Failed to fetch application state.";

    onMount(async () => {
        fetch("/app/state", {
            headers: {
                "X-App-Internal": "frontend",
            },
        })
        .then(async (res) => {
            if (res.ok) {
                const data = await res.json();
                state.set({ ...data, loading: false });
                document.title = data.app_title;
            } else {
                console.error(`ERROR ${res.status}: ${res.statusText}`);
                state.setError(STATE_FETCH_FAIL_MESSAGE);
            }
        })
        .catch((err) => {
            console.error(NETWORK_ERROR_MESSAGE, err);
            state.setError(NETWORK_ERROR_MESSAGE);
        });
    });
</script>

<Router {routes} />

{#if $state.debug}
    <DevPanel />
{/if}