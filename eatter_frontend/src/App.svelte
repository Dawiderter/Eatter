<script lang="ts">
  import Feed from './lib/Feed.svelte'
</script>

<main>
  <header>
    <h1>Eatter</h1>
  </header>
  <Feed />
</main>

<style>
  header {
    background-color: black;
    text-align: center;
    padding: 20px;
  }
  :global(body) {
    flex-direction: column;
    align-items: flex-start;
    background-color: beige;
  }

</style>