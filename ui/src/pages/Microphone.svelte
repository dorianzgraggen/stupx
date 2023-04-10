<script lang="ts">
  import { onMount } from 'svelte';
  import { init } from './microphone';

  let canvas: HTMLCanvasElement;
</script>

<h1>Microphone</h1>
<button on:click={() => init(canvas)}>los gahts</button>
<canvas bind:this={canvas} width="640" height="100" />

<style>
  canvas {
    border: 1px solid red;
  }
</style>
