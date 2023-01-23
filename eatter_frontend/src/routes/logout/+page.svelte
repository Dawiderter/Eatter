<script>
	import { invalidateAll } from "$app/navigation";
    import Button from "$lib/ui_components/Button.svelte";
</script>

<form method="POST" action="/logout">
    <Button>
        Wyloguj
    </Button>
</form>