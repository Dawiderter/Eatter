<script>
	import { invalidateAll } from "$app/navigation";
    import Button from "$lib/ui_components/Button.svelte";
</script>

<form method="POST">
    <Button on:click={() => {invalidateAll()}}>
        Wyloguj
    </Button>
</form>