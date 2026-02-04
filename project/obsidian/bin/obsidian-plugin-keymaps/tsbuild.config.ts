import { assetsPlugin, defineConfig } from "@sribich/tsbuild"

export default defineConfig({
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

/*
    plugin: {
        assets: {
            globs: [
                {
                    input: "assets",
                    glob: ".*",
                },
                {
                    input: "assets",
                    glob: "*",
                },
            ],
        },
        output: {
            assetFileNames: (assetInfo) => {
                if (assetInfo.name === "index.css") {
                    return "styles.css"
                }
                if (assetInfo.name === "index.js") {
                    return "main.js"
                }

                return assetInfo.name
            },
        },
        */
