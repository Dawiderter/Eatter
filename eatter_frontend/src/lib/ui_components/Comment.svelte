<script lang="ts">
    export let c : {c_id : number, c_body : string, c_created_at : Date, r_id : number, u_id : number, u_nick : string};
</script>

<div class = "rounded-lg w-3/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <a href="/user{c.u_id}"><h2 class = "font-jetbrains text-xl">{c.u_nick}</h2></a>
    <h3 class = "font-raleway text-xs tab-6">Date: {c.c_created_at}</h3>
    <p class = "p-3 text-l font-sans">{c.c_body}</p>
</div>