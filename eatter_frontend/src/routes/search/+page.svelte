<script lang="ts">
	import { mergeAlias } from "vite";


     export let data : import("./$types").PageData;

const meals = data.meals;
</script>

<h1>meals:</h1>
{#each meals as meal}
    meal.m_name;
{/each}