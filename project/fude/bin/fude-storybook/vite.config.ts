import stylex from "@stylexjs/unplugin"
import { defineConfig } from "vite"
import tailwindcss from "@tailwindcss/vite"

export default defineConfig({
    resolve: {
        alias: {
            "@/preview": import.meta.resolve("./.storybook/preview.tsx"),
        },
    },
    plugins: [
        tailwindcss(),
        stylex.vite({
            useCSSLayers: false,
            dev: process.env.NODE_ENV === "development",
            // treeshakeCompensation: true,
            runtimeInjection: false,
            libraries: [
                "@sribich/fude",
                "@sribich/fude-theme",
                "@sribich/fude-storybook-components",
            ],
            styleResolution: "application-order",
        }),
    ],
})
