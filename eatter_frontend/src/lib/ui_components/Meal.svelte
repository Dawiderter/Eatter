<script lang="ts">
    export let meal_item : {m_id: number, m_price: number, m_name: string, l_id: number, l_name: number};

</script>

<article>
    <a href="/local{meal_item.l_id}/meal{meal_item.m_id}">
        <h1>{meal_item.m_name}</h1>
    </a>
    <h2>{meal_item.m_price} zł</h2>
    <a href="/local{meal_item.l_id}">
        {meal_item.l_name}
    </a>

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