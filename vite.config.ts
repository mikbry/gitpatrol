import { defineConfig } from 'vite';
import path from 'path';
import { fileURLToPath } from 'url';
import wasm from 'vite-plugin-wasm';
import dts from 'vite-plugin-dts';

const filename = fileURLToPath(import.meta.url);
const dirname = path.dirname(filename);

export default defineConfig({
  resolve: {
    alias: {
      '@/gitpatrol-wasm': path.resolve(dirname, './platforms/wasm'),
      'gitpatrol': path.resolve(dirname, './webapp'),
    },
  },
  plugins: [wasm(), dts()],
});
