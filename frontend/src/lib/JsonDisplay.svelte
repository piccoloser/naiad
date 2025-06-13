<script lang="ts">
    export let data: Record<string, any>;
    const isMap = (v: any) =>
        typeof v === "object" && v !== null && !Array.isArray(v);
</script>

<div class="container">
    {#each Object.entries(data) as [key, value]}
        <b class:span="{isMap(value)}">{key}</b>
        {#if isMap(value)}
            <table>
                {#each Object.entries(value) as [k, v]}
                    <tbody>
                        <tr>
                            <td>{k}</td>
                            <td>{v}</td>
                        </tr>
                    </tbody>
                {/each}
            </table>
        {:else if typeof value === "boolean"}
            <input type="checkbox" bind:checked="{value}" readonly />
        {:else}
            <p>{value}</p>
        {/if}
    {/each}
</div>

<style>
    .container {
        display: grid;
        grid-template-columns: 1fr 2fr;
        gap: 0.25rem 0.5rem;

        max-width: 300px;
        overflow-x: hidden;
    }

    b {
        text-align: right;
        font-weight: bold;
    }

    table {
        grid-column: 1 / -1;
        border-collapse: collapse;
        width: 100%;
    }

    input[type="checkbox"] {
        margin-right: auto;
    }

    table {
        margin-bottom: 0.25rem;
        overflow: hidden;
    }

    td {
        border: solid 1px #fff;
        padding: 0.1rem 0.5rem;
    }

    td:nth-child(odd) {
        text-align: right;
        font-weight: bold;
    }

    tr {
        font-size: 0.8rem;
    }

    .span {
        grid-column: 1 / -1;
        padding-top: 0.25rem;
        text-align: center;
        border-top: solid 1px #fff;
    }
</style>
