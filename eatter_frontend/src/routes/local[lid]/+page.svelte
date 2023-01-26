<script lang="ts">
	import MealCard from "$lib/ui_components/Meal.svelte";

    export let data : import("./$types").PageServerData;

</script>

<div class = "">
	<h2 class = "pt-5 font-raleway">Menu</h2>
	{#each data.items as item}
		<MealCard 
			meal_item = {item} 
		/>
	{/each}
</div>