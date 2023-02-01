<script lang="ts">
	import ItemForm from "$lib/ui_components/ItemForm.svelte";
import Local from "$lib/ui_components/Local.svelte";

    export let data : import("./$types").LayoutData;

    let header = "Menu";
</script>

<div class = "pt-10 flex flex-col items-center justify-center ">
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
