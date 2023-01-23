<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import MealCard from "$lib/ui_components/MealCard.svelte";

    export let data : import("./$types").PageData;

    const meals = [1,2,3,4,5,6,7,8,9,10,21,3213,1224,4124,1,1234,512];

</script>

<section>
    <div>
        {#each meals as m}
            <MealCard name={m.toString()} />  
        {/each}
    </div>
</section>

<style>
    div {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }
</style>