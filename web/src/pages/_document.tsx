import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body>
        <Main />
        <NextScript />
        <p>This is a demo for <a href="https://github.com/GoogleChromeLabs/wasm-bindgen-rayon">wasm-bindgen-rayon</a>,
          generating a <a href="https://en.wikipedia.org/wiki/Mandelbrot_set">Mandelbrot fractal</a> with WebAssembly threads.
        </p>
        <input type="button" id="singleThread" value="Draw using a single thread" disabled />
        <input type="button" id="multiThread" value="Draw using all available threads" disabled />
        <output id="time"></output>
        <br />
        <canvas id="canvas" width="700" height="700"></canvas>
      </body>
    </Html>
  )
}