import wasm from 'vite-plugin-wasm-esm'
import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [sveltekit(), wasm(['shared'])],
  build: {
    target: 'esnext',
    commonjsOptions: {
      include: [/generated\/types/, /node_modules/],
    },
  },
  optimizeDeps: {
    include: [
      'shared_types',
      'shared_types/app',
      'shared_types/bincode',
      'shared_types/serde',
    ],
  },
})
