import {
    assetsPlugin,
    defineConfig,
    outputPlugin,
    stylexPlugin,
    workerPlugin,
} from "@sribich/tsbuild"

export default defineConfig({
    entrypoints: ["src/styles.ts"],
    bundle: true,
    // backend: "rolldown",
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
    sourcemap: "both",
    plugins: [
        assetsPlugin({
            globs: [
                { input: "assets", glob: ".*" },
                { input: "assets", glob: "*" },
            ],
        }),
        workerPlugin(),
        stylexPlugin(),
        outputPlugin({
            assetFileNames: (assetInfo) => {
                // This is a hack to fix stylex outputting the file int he wrong spot
                if (assetInfo.name === "styles.js") {
                    return "main.js"
                }

                // return assetInfo.name
            },
        }),
    ],
})
