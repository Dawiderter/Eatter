<script lang="ts">
	import Button from "$lib/ui_components/Button.svelte";
	import MealCard from "$lib/ui_components/MealCard.svelte";

    export let data : import("./$types").PageData;
</script>

<MealCard name="{data.mid}" />