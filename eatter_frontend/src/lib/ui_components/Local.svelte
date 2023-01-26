<script lang="ts">
    export let l : {l_id : number, l_name : string, l_phone_num : string, l_contact_email : string, l_address : string, c_id : number, c_name : string};
</script>

<div class = "rounded-lg w-3/12 bg-gray-200 border-r-2 shadow-lg p-8 ">
    <h2 class = "font-jetbrains text-xl">{l.c_name}</h2>
    <a href="/local{l.l_id}">
        <h3 class = "font-raleway text-xs tab-6">{l.l_name}</h3>
    </a>
    <ul class = "p-3 text-l font-sans">
        <li>{l.l_phone_num}</li>
        <li>{l.l_contact_email}</li>
        <li>{l.l_address}</li>
    </ul>
</div>