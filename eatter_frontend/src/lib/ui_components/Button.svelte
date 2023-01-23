<script>
</script>

<button on:click>
    <slot></slot>
</button>

<style>
    button {
        padding: 10px;
        text-align: center;
        background-color: var(--color-bg-2);
        border-radius: var(--corner-radius);
        font-weight: bold;
        border-style: none;
        width: 100%;
    }

    button:hover {
        background-color: var(--color-fg-1);
    }
</style>