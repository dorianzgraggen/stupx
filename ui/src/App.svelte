<script lang="ts">
  import { onMount } from 'svelte';
  import Paint from './pages/Paint.svelte';
  import Microphone from './pages/Microphone.svelte';
  import Index from './pages/Index.svelte';

  let canvas: HTMLCanvasElement;

  const params = new URLSearchParams(window.location.search);

  onMount(() => {});
</script>

<main>
  {#if params.has('microphone')}
    <Microphone />
  {:else if params.has('paint')}
    <Paint />
  {:else}
    <Index />
  {/if}
</main>
