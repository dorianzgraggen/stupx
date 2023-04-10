export async function init(canvas: HTMLCanvasElement) {
  const audio_context = new AudioContext();
  const analyser = audio_context.createAnalyser();
  analyser.minDecibels = -90;
  analyser.maxDecibels = -10;
  analyser.smoothingTimeConstant = 0.85;

  const canvas_context = canvas.getContext('2d');

  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    const source = audio_context.createMediaStreamSource(stream);
    source.connect(analyser);
    analyser.connect(audio_context.destination);

    process();
  } catch (error) {
    console.error(error);
  }

  // yoinked canvas stuff from here:
  // https://github.com/mdn/webaudio-examples/blob/3eb15bde7bfbab77ba8ec7abf38e00a8ae292bdc/voice-change-o-matic/scripts/app.js
  function process() {
    let WIDTH = canvas.width;
    let HEIGHT = canvas.height;

    analyser.fftSize = 256;
    const buffer_length = analyser.frequencyBinCount;
    const data_array = new Uint8Array(buffer_length);

    canvas_context.clearRect(0, 0, WIDTH, HEIGHT);

    const render = () => {
      requestAnimationFrame(render);

      analyser.getByteFrequencyData(data_array);

      canvas_context.fillStyle = 'rgb(0, 0, 0)';
      canvas_context.fillRect(0, 0, WIDTH, HEIGHT);

      const bar_width = (WIDTH / buffer_length) * 2.5;
      let bar_height: number;
      let x = 0;

      for (let i = 0; i < buffer_length; i++) {
        bar_height = data_array[i];

        canvas_context.fillStyle = 'rgb(' + (bar_height + 100) + ',50,50)';
        canvas_context.fillRect(
          x,
          HEIGHT - bar_height / 2,
          bar_width,
          bar_height / 2
        );

        x += bar_width + 1;
      }
    };

    render();
  }
}
