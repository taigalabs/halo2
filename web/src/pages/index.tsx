import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '@/styles/Home.module.css'
import React from 'react';

import * as Comlink from 'comlink';

const inter = Inter({ subsets: ['latin'] })

const state = {
  aa: false,
};

console.log('index.js');

export default function Home() {
  React.useEffect(() => {
    console.log('Home()');

    const maxIterations = 1000;

    const canvas = document.getElementById('canvas') as any;
    const { width, height } = canvas;
    const ctx = canvas!.getContext('2d');
    const timeOutput = document.getElementById('time');

    (async function init() {
      console.log('init()');

      if (state.aa === false) {
        console.log('init() initial');
        state.aa = true;
      } else {
        console.log('init() 2');
        return;
      }

      // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
      let handlers = (await Comlink.wrap(
        new Worker(new URL('/wasm-worker.mjs', import.meta.url), {
          type: 'module'
        })
      ) as any).handlers;

      console.log(444, handlers);

      function setupBtn(id: any) {
        console.log('setupBtn()', id);

        // Handlers are named in the same way as buttons.
        let handler = handlers[id];
        // If handler doesn't exist, it's not supported.
        if (!handler) return;

        // Assign onclick handler + enable the button.
        Object.assign(document.getElementById(id) as any, {
          async onclick() {
            console.log('btn clicked', handlers);
            await handler({ arg: 3 });

            // console.log('res', res);
            // console.log(555, handler);

            // let { rawImageData, time } = await handler({
            //   width,
            //   height,
            //   maxIterations
            // });

            // (timeOutput as any).value = `${time.toFixed(2)} ms`;

            // const imgData = new ImageData(rawImageData, width, height);
            // ctx.putImageData(imgData, 0, 0);
          },
          disabled: false
        });
      }

      console.log('setting up buttons');

      // setupBtn('singleThread');

      if (handlers.supportsThreads) {
        console.log('support threads!');

        setupBtn('multiThread');
      }
    })();
  }, []);

  return (
    <>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div>
        123123
      </div>
    </>
  )
}

