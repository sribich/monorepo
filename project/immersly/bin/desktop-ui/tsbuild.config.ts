import { defineConfig, reactPlugin, servePlugin, stylexPlugin } from "@sribich/tsbuild"
import { reactRouterPlugin } from "@sribich/tsbuild"

export default defineConfig({
    // entrypoints: ["src/main.tsx"],
    formats: ["esm"],
    bundle: true,
    minify: false,
    // backend: process.env.ROLLDOWN ? "rolldown" : "esbuild",
    backend: "rolldown",
    preset: "webApp",
    watch: {
        excludeGlobs: ["src/generated/**"],
    },

    /*
    plugin: {
        serve: {

        },
        html: {
            htmlTemplate: "./src/index.html",
        },
    },
    */
    server: {
        port: 4445,
        proxy: {
            "/play": { target: "http://127.0.0.1:7057" },
            "/rpc": { target: "http://127.0.0.1:7057" },
        },
    },
    plugins: [reactPlugin(), stylexPlugin()],
})
