import { defineConfig } from "@sribich/tsbuild"

export default defineConfig({
    entrypoints: ["src/index.ts"],
    formats: ["esm", "cjs"],
})
