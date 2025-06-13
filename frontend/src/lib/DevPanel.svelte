<script lang="ts">
    import { onMount } from "svelte";
    import { state } from "../stores";
    import JsonDisplay from "./JsonDisplay.svelte";

    let authProvider: string | null = null;
    let container: HTMLDivElement | null = null;
    let floating: boolean = false;
    let minimized: boolean = true;
    
    const isCtrlAltShift = (e: KeyboardEvent) =>
        e.ctrlKey && e.altKey && e.shiftKey;

    const handleKeydown = (event: KeyboardEvent) => {
        if (!isCtrlAltShift(event)) return;
        if (event.key === "C") toConfigEditor();
        if (event.key === "D") toggleMinimized();
    };

    const toConfigEditor = () => {
        window.location.href = "/config";
    };

    const toggleMinimized = () => {
        minimized = !minimized;
        localStorage.setItem("devpanel-minimized", minimized.toString());
    };

    onMount(() => {
        (async () => {
            let authProviderRes = await fetch("/app/auth_info", {
                headers: { "X-App-Internal": "frontend" }
            });

            if (!authProviderRes.ok) {
                console.error("Failed to fetch auth provider info");
                return;
            }

            authProvider = await authProviderRes.json();
            console.log("Auth Provider:", authProvider);
        })();

        const storedState = localStorage.getItem("devpanel-minimized");
        minimized = storedState === null ? false : storedState === "true";
        const storedFloating = localStorage.getItem("devpanel-floating");
        floating = storedFloating === null ? false : storedFloating === "true"

        window.addEventListener("keydown", handleKeydown);
        return () => {
            window.removeEventListener("keydown", handleKeydown);
        };
    });

    $: appState = $state;
</script>

<div class="container" bind:this={container} class:minimized class:floating>
    <div class="header">
        <p>Dev Panel {floating ? "(Floating)" : ""}</p>

        <span>
            <button
                class="header-btn"
                on:click={toConfigEditor}
                title="To Config Editor"
            >
                &#9881;
            </button>
            <button
                class="header-btn"
                on:click={toggleMinimized}
                title="Toggle Panel (Ctrl+Alt+Shift+D)"
            >
                {minimized ? "\u25B2" : "\u25BC"}
            </button>
        </span>
    </div>

    <section>
        <b>Application State</b>
        <JsonDisplay data={{...appState, auth_provider: authProvider}} />
    </section>
</div>

<style>
    :root {
        --dp-header-height: 2rem;
    }

    .container {
        position: absolute;
        top: calc(var(--header-height));
        right: 0;

        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        height: var(--body-height);
        width: 300px;
        overflow-y: auto;

        background-color: #000a;
        color: #fff;
    }

    div.minimized {
        position: fixed;
        top: calc(100vh - var(--dp-header-height));
        right: 0;
    }

    section {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        padding: 0.5rem;
    }

    .floating {
        position: fixed;
        top: var(--header-height);
        right: 0;
    }

    .header {
        position: relative;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        gap: 0.5rem;
        padding: 0.5rem;
        height: var(--dp-header-height);
    }

    .header-btn {
        padding: 0;
        background: none;
        border: none;
        color: #fff;
        cursor: pointer;
        user-select: none;
    }
</style>
