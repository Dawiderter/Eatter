<script lang="ts">
    export let data : import("./$types").LayoutData;
</script>

<div>
    <h1>Local {data.lid}</h1>
    <slot></slot>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>