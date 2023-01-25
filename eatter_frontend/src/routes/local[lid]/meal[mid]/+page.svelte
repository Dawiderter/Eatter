<script lang="ts">
	import MealCard from "$lib/ui_components/Meal.svelte";

    export let data : import("./$types").PageServerData;

</script>

<MealCard 
	meal_item = {data.item} 
/>