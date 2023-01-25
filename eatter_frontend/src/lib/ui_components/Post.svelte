<script lang="ts">
    export let author : string;
    export let body : string;
    export let local_name : string
    export let meal_name : string
    export let score : number;
    export let created_at : Date

    const inner_date = created_at.toLocaleString('en-GB', { timeZone: 'UTC' });
</script>

<div class = "w-4/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <h1 class = "text-2xl">{author}</h1>
    <h2>Score: {score}</h2>
    <h2>Date: {inner_date}</h2>
    <h2>Place: {local_name}</h2>
    <h3>Meal: {meal_name}</h3>
    {body},
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