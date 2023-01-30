<script lang="ts">
	import Meal from "$lib/ui_components/Meal.svelte";

    export let data : import("./$types").PageData;
</script>

{#each data.meals as meal}
    <Meal meal_item = {meal}></Meal>
{/each}