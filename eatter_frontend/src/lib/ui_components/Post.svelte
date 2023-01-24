<script lang="ts">
    export let author : string;
    export let body : string;
    export let meal : string;
    export let score : number;
    export let comments_num : number;
</script>

<div>
    <h2>{author}</h2>
    <h3>Meal: {meal}</h3>
    {body},
    <h3>Score: {score}</h3>
</div>



<style>
    div {
        width: 500px;
        display: flex;
        flex-direction: column;
        padding: 10px;
        margin: 10px;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    }
</style>