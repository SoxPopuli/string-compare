import { defineConfig } from 'vite'

export default defineConfig({
    base: '', //use relative paths in build
    plugins: [],

    optimizeDeps: {
        exclude: [ "string_compare_bg.wasm" ]
    }
})
