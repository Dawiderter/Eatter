<script>
	export let value = '';
	export let minRows = 1;
	export let maxRows = 3;
	export let name = "text_input";
    export let maxCharacters = 150;

    let custom_class = "";
	export { custom_class as class };

    custom_class = custom_class + " container flex";

    let comment_input_body = "";
	
	$: minHeight = `${1 + minRows * 1.2}em`;
	$: maxHeight = maxRows ? `${1 + maxRows * 1.2}em` : `auto`;
</script>

	<textarea class = "resize-none border border-gray-300 rounded-lg bg-gray-50" name={name} maxlength={maxCharacters} bind:value></textarea>	
	
<style>
	.container {
		position: relative;
	}
	
	pre, textarea {
		font-family: inherit;
		padding: 0.5em;
		box-sizing: border-box;
		border: 1px solid #eee;
		line-height: 1.2;
		overflow: hidden;
	}
</style>