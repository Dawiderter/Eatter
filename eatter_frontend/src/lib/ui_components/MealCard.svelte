<script lang="ts">
    export let name : string;
</script>

<article on:click>
    <h1>{name}</h1>
    <p>Lorem ipsum</p>
</article>

<style>
    article {
        display: flex;
        flex-direction: column;
        text-align: center;
        padding: 10px;
        margin: 10px;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    }
</style>