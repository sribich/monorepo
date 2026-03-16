import { defineConfig } from "astro/config"
import starlight from "@astrojs/starlight"
import { URL, fileURLToPath } from "node:url"
import { join } from "node:path"

export default defineConfig({
    integrations: [
        starlight({
            title: "Railgun",
            social: [],
            sidebar: [
                {
                    label: "Introduction",
                    slug: "introduction",
                },
                {
                    label: "Getting Started",
                    items: [],
                },
                {
                    label: "Packages",
                    items: [],
                },
            ],
        }),
    ],
})
