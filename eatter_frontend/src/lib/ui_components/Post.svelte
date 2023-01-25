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

    const inner_date = new Date(created_at);

    const post_link = "/post" + post_id;
</script>


<div class = "w-4/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    
    <h1 class = "font-jetbrains text-2xl">{author}</h1>
    <h2 class = "font-jetbrains text-xs tab-6">Date: {inner_date.getMonth()} &emsp; | &emsp; Place: {local_name} &emsp; | &emsp; Score: {score}</h2>
    <h3 class = "text-xl">Meal: {meal_name}</h3>
    <p class = "jetbrains ">{body}</p>
    <a href={post_link}>Check comments</a>
</div>



<style>
    /* div {
        width: 500px;
        display: flex;
        flex-direction: column;
        padding: 10px;
        margin: 10px;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    } */
</style>