<script lang="ts">
    export let post_id: number;
    export let meal_id: number;
    export let author_id: number;
    export let author : string;
    export let body : string;
    export let local_name : string
    export let meal_name : string
    export let score : number;
    export let created_at : Date
    export let check_comments_visible = true;

    const inner_date = new Date(created_at);

    const post_link = "/post" + post_id;

    const stars = '★'.repeat(score);
</script>


<div class = "rounded-lg w-4/12 bg-gray-200 border-r-2 shadow-lg p-8">
    <h1 class = "font-jetbrains font-bold text-2xl">{author} → {meal_name}</h1>
    <h2 class = "font-raleway text-xs tab-6">{inner_date.getMonth()} &emsp; | &emsp;
    {local_name} &emsp; | &emsp;
    {stars}</h2>
    <p class = "p-5 text-l">{body}</p>
    {#if check_comments_visible}
    <a class = "font-jetbrains font-bold" href={post_link}>Check comments</a>
    {/if}
</div>