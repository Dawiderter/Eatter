<script>
	import Icon from "./Icon.svelte";
	import Bar from "./Bar.svelte";
	import Menu from "./Menu.svelte";
	export let sidebar = false;

</script>
<header class="fixed top-0 w-full flex justify-between bg-gray-200 p-2 items-center text-gray-600 border-b-2">
	<nav class="flex">
		<Icon bind:open={sidebar}/>
		<Bar/>	
	</nav>
	<h1 class="font-mono text-6xl font-bold center">Eatter</h1>
	
	<Menu/>
</header>