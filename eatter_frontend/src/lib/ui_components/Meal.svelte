<script lang="ts">
    export let meal_item : {m_id: number, m_price: number, m_name: string, l_id: number, l_name: number};

    export let selected = false;

</script>

<article>
    {#if selected}
        <a class = "text-xl font-bold" href="/local{meal_item.l_id}">
            <h1>{meal_item.m_name}</h1>
        </a>
    {:else}
        <a class = "text-xl font-bold" href="/local{meal_item.l_id}/meal{meal_item.m_id}">
            <h1>{meal_item.m_name}</h1>
        </a>
    {/if}
    <h2 class = "font-raleway">{meal_item.m_price} zł
    <a href="/local{meal_item.l_id}"> &emsp; | &emsp;
        {meal_item.l_name}
    </a> </h2>

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