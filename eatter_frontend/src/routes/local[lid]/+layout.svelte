<script lang="ts">
	import ItemForm from "$lib/ui_components/ItemForm.svelte";
import Local from "$lib/ui_components/Local.svelte";

    export let data : import("./$types").LayoutData;

    let header = "Menu";
</script>

<div class = "pt-10 flex flex-col items-center justify-center ">
    <Local l={data.local}/>
    {#if data.auth != null && data.auth.company_id != null && data.auth.company_id == data.local.c_id}
        <ItemForm action="?/add" fields={[{name: "name", type: "text"}, {name: "price", type: "text"}]}></ItemForm>
    {/if}
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
