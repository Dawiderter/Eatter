<script lang="ts">
    import { page } from '$app/stores';
	import ItemForm from "$lib/ui_components/ItemForm.svelte";
	import Local from '$lib/ui_components/Local.svelte';

    import type { PageData } from "./$types";

    export let data : PageData;
</script>

<div class = "flex flex-col items-center justify-center">
    <h1 class = "mt-[10px] p-10 text-6xl font-raleway">Manage your company</h1>
    <h2 class = "pt-5 text-2xl font-raleway">Your locals:</h2>
    <ItemForm action="?/add" fields={[
        {name: "name", type: "text"},
        {name: "phone_number", type: "text"},
        {name: "contact_email", type: "text"},
        {name: "address", type: "text"}]}>
    </ItemForm>
    {#each data.locals as local}
    <div class = "flex flex-row w-screen items-center justify-center p-5">
        <form action="?/del" method="post">
            <input type="hidden" name="local_id" value="{local.l_id}">
            <button class="p-5">
                <span class="close">+</span>
            </button>
        </form>
        <Local l = {local}></Local> 
    </div>
    {/each}
</div>

<style>
    .close {
  font-size: 55px;
  font-weight: 200;
  display: inline-block;
  transform: rotate(45deg);
}
</style>