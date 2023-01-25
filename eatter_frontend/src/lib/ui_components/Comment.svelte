<script lang="ts">
    export let author : string;
    export let body : string;
    export let created_at : Date;
</script>

<div class = "w-4/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <h2 class = "font-jetbrains text-xl">{author}</h2>
    <h3 class = "font-raleway text-xs tab-6">Date: {created_at}</h3>
    {body}
</div>