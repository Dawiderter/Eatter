<script>
	import { invalidateAll } from "$app/navigation"

</script>
<form method="post">
    <div>
        <label for="email">Email</label>
        <input name="email" type="email">
    </div>
    <div>
        <label for="pass">Password</label>
        <input name="pass" type="password">
    </div>
    <div>
        <label for="nick">Nickname</label>
        <input name="nick" type="text">
    </div>
    <div>
        <button class = "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">
            Register
        </button>
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
</style>