import { defineConfig } from "@sribich/tsbuild"

export default defineConfig({
    build: {
        preset: "webLib",
        entrypoints: ["src/index.ts"],
        formats: ["esm", "cjs"],
    },
    plugin: {
        stylex: false,
    },
})
