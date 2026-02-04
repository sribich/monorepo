import { join } from "path"
import type { Config } from "tailwindcss"

export default {
    content: [join(__dirname, "./src/**/*.{ts,tsx}")],
    corePlugins: {
        preflight: false,
    },
    theme: {},
    plugins: [require("tailwindcss-animate")],
} satisfies Config
