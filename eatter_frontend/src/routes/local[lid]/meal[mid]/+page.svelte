<script lang="ts">
	import MealCard from "$lib/ui_components/Meal.svelte";
	import Post from "$lib/ui_components/Post.svelte";
	import ResizableInput from "$lib/ui_components/ResizableInput.svelte";

    export let data : import("./$types").PageData;
</script>

{#if data.auth != null && data.auth.company_id != null && data.auth.company_id == data.local.c_id}
<h1 class = "text-2xl font-raleway pt-10">Add tags:</h1>
	<form action="?/add_tag" method="post" class = "w-3/12 pt-10 pb-5">
		<label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
		<div class="relative">
			<input type="text" name="tag" id="default-search" class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500" placeholder="Add tags..." required>
			<button class="text-white absolute right-1.5 bottom-2.5 bg-gray-600 hover:bg-gray-700 focus:ring-4 focus:outline-none font-xs rounded-lg text-sm px-1 py-2 ">Add Tag</button>
		</div>
	</form>
{/if}
<h1 class = "text-2xl font-raleway">Current tags:</h1>
<ul class = "p-1 flex flex-col items-center justify-center">
	{#each data.tags as tag}
		<li class = "flex flex-row items-center mt-[5px] rounded-lg bg-gray-200 border-r-2 shadow-sg w-fit p-1">
			{#if data.auth != null && data.auth.company_id != null && data.auth.company_id == data.local.c_id}
				<form action="?/del_tag" method="post">
					<button>
						<span class="close">+</span>
					</button>
					<input name="tag" type = "hidden" value="{tag.name}">
				</form>
			{/if}
			{tag.name}
		</li>
	{/each}
</ul>
<div class = "relative">
	<MealCard selected = {true} meal_item = {data.item} />
	{#if data.auth && data.auth.company_id == null}
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
	.close {
		font-size: 25px;
		font-weight: 200;
		display: inline-block;
		transform: rotate(45deg);
	}
</style>