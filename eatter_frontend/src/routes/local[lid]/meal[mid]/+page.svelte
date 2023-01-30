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
		<form class="input-group z-0" method="post" action="?/add">
			Write a review:
			<ResizableInput name="review_body" class = "pt-2 w-8/12" minRows={2} maxRows={5}/>
			<div class="rate">
				<input type="radio" id="star5" name="rate" value="5" />
				<label for="star5" title="text">5 stars</label>
				<input type="radio" id="star4" name="rate" value="4" />
				<label for="star4" title="text">4 stars</label>
				<input type="radio" id="star3" name="rate" value="3" />
				<label for="star3" title="text">3 stars</label>
				<input type="radio" id="star2" name="rate" value="2" />
				<label for="star2" title="text">2 stars</label>
				<input type="radio" id="star1" name="rate" value="1" />
				<label for="star1" title="text">1 star</label>
			</div>
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
	.rate {
		float: left;
		height: 46px;
		padding: 0 10px;
	}
	.rate:not(:checked) > input {
		position:absolute;
		top:-9999px;
	}
	.rate:not(:checked) > label {
		float:right;
		width:1em;
		overflow:hidden;
		white-space:nowrap;
		cursor:pointer;
		font-size:30px;
		color:#ccc;
	}
	.rate:not(:checked) > label:before {
		content: '★ ';
	}
	.rate > input:checked ~ label {
		color: #ffc700;    
	}
	.rate:not(:checked) > label:hover,
	.rate:not(:checked) > label:hover ~ label {
		color: #deb217;  
	}
	.rate > input:checked + label:hover,
	.rate > input:checked + label:hover ~ label,
	.rate > input:checked ~ label:hover,
	.rate > input:checked ~ label:hover ~ label,
	.rate > label:hover ~ input:checked ~ label {
		color: #c59b08;
	}
</style>