<script lang="ts">
    export let u : {u_id : number, u_nick : string, u_bio: string | null};
</script>

<div class = "rounded-lg w-4/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <a href="/user{u.u_id}"><h2 class = "font-jetbrains text-xl">{u.u_nick}</h2></a>
    {#if u.u_bio != null}
        Bio: {u.u_bio}
    {/if}
</div>