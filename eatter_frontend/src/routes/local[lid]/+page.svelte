<script lang="ts">
	import MealCard from "$lib/ui_components/Meal.svelte";

    export let data : import("./$types").PageData;

</script>

<div class = "">
	
	{#each data.items as item}
	<div class = "flex flex-row w-screen  items-center justify-center">
		{#if data.auth != null && data.auth.company_id != null}
			<form action="?/del" method="post">
				<input type="hidden" name="meal_id" value="{item.m_id}">
				<button class="p-5">
					<span class="close">+</span>
				</button>
			</form>
		{/if}
			<MealCard 
				meal_item = {item} 
			/>
		</div>
	{/each}
</div>

<style>
    .close {
  font-size: 25px;
  font-weight: 200;
  display: inline-block;
  transform: rotate(45deg);
}
</style>