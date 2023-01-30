<script lang="ts">
    export let action : string;
    export let fields : {name: string, type: string}[];
</script>

<form method="post" action={action} class = "w-fit bg-gray-200 p-5 mt-[15px] flex flex-col rounded-3xl items-center justify-center gap-2">
    <h2 class = "p-5 text-2xl font-raleway">Add new item:</h2>
    {#each fields as field}
        <div>
            <label for={field.name}>{field.name}</label>
            <input name={field.name} type={field.type}>
        </div>
    {/each}
    <button class = "bg-gray-600 hover:bg-gray-700 w-fit p-10 text-white font-bold py-2 px-4 rounded-full">Add</button>
</form>