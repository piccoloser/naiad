<script lang="ts">
    import ConfigEditor from "./ConfigEditor.svelte";
    export let data: Record<string, any>;
</script>

<ul>
    {#each Object.entries(data) as [key, value]}
    <li>
        {#if typeof value === "object" && value !== null && !Array.isArray(value)}
        <fieldset>
            <legend>{key}</legend>
            <ConfigEditor bind:data={data[key]} />
        </fieldset>
        {:else}
        <label for={key}>{key}:</label>
        <input name={key} type={typeof value === "number" ? "number" : "text"} bind:value={data[key]} />
        {/if}
    </li>
    {/each}
</ul>

<style>
    fieldset {
        border: none;
        border-top: solid 1px #aaa8;
        padding: 0.5rem 0.5rem 0.25rem;
        margin: 0.5rem 0;
        text-align: left;
    }

    input {
        min-width: 24ch;
    }

    label {
        display: inline-block;
        min-width: 14ch;
        max-width: max-content;
        margin: 0.25rem 0;
    }

    legend {
        text-transform: uppercase;
        font-size: 1.2rem;
        font-weight: bold;
        padding: 0 0.5rem;
    }

    ul {
        list-style: none;
    }
</style>