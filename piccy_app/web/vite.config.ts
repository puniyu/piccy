import {defineConfig} from "vite";
import react from '@vitejs/plugin-react'
import tsconfigPaths from "vite-tsconfig-paths";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";
import {fileURLToPath} from "node:url";


const filePath = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
    clearScreen: false,
    server: {
        host: "0.0.0.0",
        port: 33720,
        strictPort: true,
    },
    plugins: [
        react(),
        tsconfigPaths(),
        tailwindcss(),
    ],
    build: {
        target: process.env.TAURI_ENV_PLATFORM == 'windows'
            ? 'chrome105'
            : 'safari13',
        outDir: path.join(filePath, "dist"),
        cssCodeSplit: true,
        chunkSizeWarningLimit: 1000,
        minify: !process.env.TAURI_ENV_DEBUG,
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
});
