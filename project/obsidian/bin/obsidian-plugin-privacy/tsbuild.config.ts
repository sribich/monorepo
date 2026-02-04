import { assetsPlugin, defineConfig } from "@sribich/tsbuild"

export default defineConfig({
    preset: "webLib",
    entrypoints: ["src/main.ts"],
    bundle: true,
    formats: ["cjs"],
    externals: [
        "obsidian",
        "electron",
        "@codemirror/autocomplete",
        "@codemirror/collab",
        "@codemirror/commands",
        "@codemirror/language",
        "@codemirror/lint",
        "@codemirror/search",
        "@codemirror/state",
        "@codemirror/view",
        "@lezer/common",
        "@lezer/highlight",
        "@lezer/lr",
        // Node builtins
        "crypto",
        "os",
        "path",
    ],
    sourcemap: "inline",
    plugins: [
        assetsPlugin({
            globs: ["manifest.json"],
        }),
    ],
})
