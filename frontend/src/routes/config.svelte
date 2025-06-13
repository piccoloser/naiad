<script lang="ts">
    import { onMount } from "svelte";
    import Header from "../components/Header.svelte";
    import ConfigEditor from "../lib/ConfigEditor.svelte";

    let config: Record<string, any> | null = null;
    let storedLdapConfig: Record<string, any> | null = null;
    let error: string | null = null;

    const deepCompare = (a: any, b: any): boolean => JSON.stringify(a) === JSON.stringify(b);

    onMount(async () => {
        try {
            const res = await fetch("/app/config", { 
                headers: { "X-App-Internal": "frontend" }
            });

            if (!res.ok) {
                throw new Error(
                    res.status === 401 ? "Not Logged In" :
                    res.status === 403 ? "Access Denied" :
                    `Unexpected error: ${res.status}`
                );
            }

            config = await res.json();

            if (config !== null)
                storedLdapConfig = structuredClone(config["sections"]["ldap"] || null);

        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
            alert(error);
            window.location.href = "/";
        }
    });

    const save = async () => {
        let clearSessions = false;

        if (!confirm("Save changes and restart the server?")) return;
        if (!deepCompare(config!["sections"]["ldap"], storedLdapConfig)) {
            if (!confirm("Modifying authentication settings will end all sessions. Are you sure you want to continue?"))
                return;

            clearSessions = true;    
        }

        const res = await fetch("/app/config", {
            method: "POST",
            headers: { "X-App-Internal": "frontend" },
            body: JSON.stringify(config)
        });

        if (!res.ok) {
            alert(`Failed to save configuration: ${res.status} ${res.statusText}`);
            return;
        }

        if (res.ok) {
            alert("Configuration saved. Restarting the server...");
            
            if (clearSessions) {
                const updateAuthRes = await fetch("/app/update_auth_provider", {
                    method: "POST",
                    headers: { "X-App-Internal": "frontend" }
                });
                if (!updateAuthRes.ok) {
                    alert(`Failed to update authentication provider: ${updateAuthRes.status} ${updateAuthRes.statusText}`);
                    return;
                }

                const clearRes = await fetch("/auth/clear_sessions", { method: "POST" });
                if (!clearRes.ok) {
                    alert(`Failed to clear sessions: ${clearRes.status} ${clearRes.statusText}`);
                    return;
                }
            }

            await fetch("/server/restart");
            window.location.reload();
        } else {
            alert(`Failed to save configuration: ${res.status} ${res.statusText}`);
        }
    }
</script>

<Header>
    <a href="/" class="btn">Home</a>
</Header>

{#if config !== null}
    <div class="cfg-editor">
        <h1>Configuration Editor</h1>
        <ConfigEditor bind:data={config} />
        <button on:click={save}>Save &amp; Restart</button>
    </div>
{/if}

<style>
    .btn {
        background-color: var(--fg);
        color: var(--bg);
    }
    .btn:hover {
        background-color: var(--fg-muted);
    }

    .cfg-editor {
        margin: 0.5rem;
        padding: 0.5rem;
        max-width: 500px;
        border: solid 1px #aaa8;
        text-align: center;
    }

    .cfg-editor button {
        display: block;
        margin-left: auto;
    }
</style>