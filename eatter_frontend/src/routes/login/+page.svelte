<script>
	import { invalidateAll } from "$app/navigation";


	import Button from "$lib/ui_components/Button.svelte";

</script>
<form method="post" action="/login">
    <div>
        <label for="email">Email</label>
        <input name="email" type="email">
    </div>
    <div>
        <label for="pass">Hasło</label>
        <input name="pass" type="password">
    </div>
    <div>
        <Button>
            Zaloguj
        </Button>
    </div>
</form>

<style>
    form {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
    div {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 10px;
    }
    input {
        border-radius: var(--corner-radius);
        border-style: none;
        padding: 5px;
        background-color: var(--color-bg-1);
    }
</style>