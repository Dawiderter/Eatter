<script lang="ts">
    let input_tag = "";
    let tags : string[];
    tags = ['spaghetti', 'bolognese'];
    $: formatted_tags = tags.join(',')
    const onclickfunc = () => {
        tags.push(input_tag);
        input_tag = "";
        tags = tags;
    };
</script>

<div class = "flex flex-col items-center justify-center">
    <div class = "w-3/12 pt-10 pb-5">
        <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
        <div class="relative">
            <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                <svg aria-hidden="true" class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
            </div>
            <input type="search" bind:value={input_tag} id="default-search" class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500" placeholder="Add tags..." required>
            <button on:click={onclickfunc} class="text-white absolute right-1.5 bottom-2.5 bg-gray-600 hover:bg-gray-700 focus:ring-4 focus:outline-none font-xs rounded-lg text-sm px-1 py-2 ">Add Tag</button>
        </div>
    </div>
    <a href = '/search/results?tags={formatted_tags}' class = "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">
        Search
    </a>
    <h1 class = "mt-[15px] text-xl font-raleway">Current tags</h1>
    <ul class = "p-1 flex flex-col items-center justify-center">
        {#each tags as tag, i (i)}
            <li class = "mt-[5px] rounded-lg bg-gray-200 border-r-2 shadow-sg w-fit p-1">
                <button on:click={() => {
                    tags.splice(i, 1);
                    tags = tags;
                }}>
                    <span class="close">+</span>
                </button>
                {tag}
            </li>
        {/each}
    </ul>
    <slot></slot>
</div>

<style>
    .close {
  font-size: 15px;
  font-weight: 200;
  display: inline-block;
  transform: rotate(45deg);
}
</style>