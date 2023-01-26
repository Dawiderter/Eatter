<script>
	import { invalidateAll } from "$app/navigation";


</script>
<form method="post" action="/login">
    <div>
        <label for="email">Email</label>
        <input name="email" type="email">
    </div>
    <div>
        <label for="pass">Password</label>
        <input name="pass" type="password">
    </div>
    <div>
        <button class = "bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded-full">
            Login
        </button>
        <a href = "/register" class = "font-bold py-2 px-4 rounded-full">
            Create account
        </a>
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