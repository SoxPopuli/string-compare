import { defineConfig } from 'vite'

export default defineConfig({
    plugins: [],

    optimizeDeps: {
        exclude: [ "string_compare_bg.wasm" ]
    }
})
