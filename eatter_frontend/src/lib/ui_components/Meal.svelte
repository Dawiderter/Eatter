<script lang="ts">
    export let meal_item : {m_id: number, m_price: number, m_name: string, l_id: number, l_name: number, m_r_num: number, m_r_avg: number | null};

    export let selected = false;
    
    let stars = meal_item.m_r_avg == null ? "?" : "★".repeat(meal_item.m_r_avg) + (meal_item.m_r_avg % 1 >= 0.5 ? "☆" : "");

</script>

<article>
    {#if selected}
        <a class = "text-xl font-bold" href="/local{meal_item.l_id}">
            <h1>{meal_item.m_name}</h1>
        </a>
    {:else}
        <a class = "text-xl font-bold" href="/local{meal_item.l_id}/meal{meal_item.m_id}">
            <h1>{meal_item.m_name}({meal_item.m_r_num})</h1>
        </a>
    {/if}
    <h2 class = "font-raleway">{meal_item.m_price} zł
    <a href="/local{meal_item.l_id}"> &emsp; | &emsp;
        {meal_item.l_name}
    </a>  &emsp; | &emsp; avg: {stars} </h2>

</article>

<style>
    article {
        display: flex;
        flex-direction: column;
        text-align: center;
        padding: 10px;
        margin: 10px;
        background-color: var(--color-bg-0);
        border-radius: var(--corner-radius);
    }
</style>