import { cp, open } from "fs/promises"
import { join, resolve } from "path"
import { defineConfig } from "vite"

export default defineConfig({
    build: {
        lib: {
            entry: resolve(__dirname, "src/main.ts"),
            fileName: () => "main.js",
            formats: ["cjs"],
        },
        rollupOptions: {
            external: [
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
                "crypto",
                "os",
                "path",
            ],
            worker: {
                inlineLimit: 0,
            },
        },
    },
    esbuild: {
        minifyIdentifiers: false,
        keepNames: true,
    },
    plugins: [
        (() => {
            let config

            return {
                name: "copy",
                apply: "build",
                configResolved: (resolvedConfig) => {
                    config = resolvedConfig
                },
                async writeBundle() {
                    cp(
                        join(config.root, "manifest.json"),
                        join(config.build.outDir, "manifest.json"),
                    )
                    const reloadFile = await open(join(config.build.outDir, ".hotreload"), "a")
                    await reloadFile.close()
                },
            }
        })(),
    ],
})
