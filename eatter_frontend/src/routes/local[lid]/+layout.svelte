<script lang="ts">
	import Local from "$lib/ui_components/Local.svelte";

    export let data : import("./$types").LayoutData;
</script>

<div class = "pt-10 flex flex-col items-center justify-center">
    <Local l={data.local}/>
    <slot></slot>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>
