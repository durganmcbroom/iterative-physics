import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
    base: process.env.REPO_NAME ? `/${process.env.REPO_NAME.split('/')[1]}/` : '/',
    plugins: [react(), tailwindcss(), wasm(), topLevelAwait()],
})
