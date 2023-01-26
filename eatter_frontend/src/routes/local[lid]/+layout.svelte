<script lang="ts">
	import Local from "$lib/ui_components/Local.svelte";

    export let data : import("./$types").LayoutData;
</script>

<div class = "pt-10 flex flex-col items-center justify-center">
    <div class = "rounded-3xl w-3/12 bg-gray-200 border-r-2 shadow-lg p-8">
        <h1 class = "p-5 text-4xl font-jetbrains font-bold">{data.local.l_name}</h1>
        <slot></slot>
    </div>
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>
