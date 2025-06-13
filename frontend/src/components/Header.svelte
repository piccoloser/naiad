<script lang="ts">
    import { state } from "../stores";
    $: appState = $state;
</script>

<header>
    <a class="title-block" href="/">
        <img src="/logo.svg" alt="[Logo]" />
        <span class="title-text">
            <h1>{appState.app_title}</h1>
            <p>
                {`${appState.app_version}${appState.debug ? " (Debug)" : ""}`}
            </p>
        </span>
    </a>
    
    <div class="buttons">
        <slot></slot>
    </div>
</header>

<style>
    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;

        padding: 0.5rem 1rem;
        height: var(--header-height);

        background-color: var(--accent);
        z-index: 99;
    }
    
    img {
        height: 100%;
        width: auto;
    }

    .buttons {
        display: grid;
        grid-auto-flow: row;
        gap: 2px;
    }

    .title-block {
        display: flex;
        align-items: center;
        gap: 0.75rem;

        height: 100%;
    }

    .title-text {
        display: flex;
        align-items: baseline;
        gap: 0.5rem;
        white-space: nowrap;
        width: max-content;
        color: #fff;
    }
</style>
