<script lang="ts">
    export let p : {r_id : number, r_body: string, r_created_at: Date, r_score: number, u_id : number, u_nick : string, m_id: number, m_name: number, l_id: number, l_name: string};

    export let comments = true;

    const stars = '★'.repeat(p.r_score);
</script>


<div class = "rounded-lg w-4/12 bg-gray-200 border-r-2 shadow-lg p-8">
    
        <h1 class = "font-jetbrains font-bold text-2xl">
            <a href="/user{p.u_id}">{p.u_nick}</a> → <a href="/local{p.l_id}/meal{p.m_id}">{p.m_name}</a> 
        </h1>
    
    <h2 class = "font-raleway text-xs tab-6">{p.r_created_at} &emsp; | &emsp;
    <a href="/local{p.l_id}">{p.l_name}</a> &emsp; | &emsp;
    {stars}</h2>
    <p class = "p-5 text-l font-sans">{p.r_body}</p>
    {#if comments}
    <a class = "font-jetbrains font-bold" href="/post{p.r_id}">Check comments</a>
    {/if}
</div>