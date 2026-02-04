import { defineConfig, reactPlugin, reactRefreshPlugin } from "@sribich/tsbuild"

export default defineConfig({
    preset: "webLib",
    backend: "rolldown",
    entrypoints: [],
    formats: ["esm"],
    plugins: [reactPlugin(), reactRefreshPlugin()],

    // plugin: {
    //     stylex: false,
    //     reactCompiler: false,
    // },
})
