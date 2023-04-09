<script lang="ts">
  import { onMount } from 'svelte';
  import { init } from './canvas';

  let canvas: HTMLCanvasElement;

  onMount(() => {
    init(canvas);
  });
</script>

<main>
  <canvas bind:this={canvas} />
</main>
