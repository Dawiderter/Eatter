<script lang="ts">
	import Local from "$lib/ui_components/Local.svelte";

    export let data : import("./$types").LayoutData;

    let header = "Menu";
</script>

<div class = "pt-10 flex flex-col items-center justify-center ">
    <Local l={data.local}/>
    <h3 class = "pt-8 text-xl font-raleway">Menu:</h3>
    <slot></slot>
    
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>
