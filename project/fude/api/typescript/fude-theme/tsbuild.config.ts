import { declarationPlugin, defineConfig } from "@sribich/tsbuild"

export default defineConfig({
    preset: "webLib",
    entrypoints: ["src/index.js"],
    formats: ["esm", "cjs"],
    plugins: [declarationPlugin()],
})
