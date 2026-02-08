import { declarationPlugin, defineConfig } from "@sribich/tsbuild"

export default defineConfig({
    preset: "webLib",
    entrypoints: ["src/index.ts"],
    formats: ["esm", "cjs"],
    plugins: [declarationPlugin()]
})
