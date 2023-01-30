<script lang="ts">
    export let l : {l_id : number, l_name : string, l_phone_num : string, l_contact_email : string, l_address : string, c_id : number, c_name : string};
</script>

<div class = "rounded-3xl w-3/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <h2 class = "p-5 text-4xl font-jetbrains font-bold">{l.c_name}
        <a href="/local{l.l_id}">
            <p class = "font-raleway text-xs tab-6">@{l.l_name}</p>
        </a>
    </h2>
    <p>{l.l_phone_num} | {l.l_contact_email} | {l.l_address}</p>
    <!-- <div class = "rounded-3xl w-3/12 bg-gray-200 border-r-2 shadow-lg p-8">
        <h1 class = "p-5 text-4xl font-jetbrains font-bold">{data.local.l_name}</h1>
        <h2 class = "pt-5 font-raleway">{header}</h2>
    </div> -->
</div>