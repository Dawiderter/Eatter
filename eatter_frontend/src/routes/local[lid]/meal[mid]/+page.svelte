<script lang="ts">
	import MealCard from "$lib/ui_components/Meal.svelte";
	import Post from "$lib/ui_components/Post.svelte";
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";

    export let data : import("./$types").PageData;

</script>
<div class = "relative">
	<MealCard selected = {true} meal_item = {data.item} />
	{#if data.auth}
	<div>
		<form class="input-group z-0" method="post">
			Write a review:
			<ResizableInput name="review_body" class = "pt-2 w-8/12" minRows={2} maxRows={5}/>
			<input type="range" name="review_score" min="0" max="5">
			<button class = "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">Post</button>
		</form>
	</div>
	{/if}
	<div class = "p-5 flex flex-col items-center justify-center space-y-5">
        <h1 class = "mt-[10px] mb-[10px] text-2xl font-raleway">Reviews:</h1>
        {#each data.reviews as p}
            <Post size="w-10/12" p={p} />
        {/each}
    </div>
</div>

<style>
    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
</style>