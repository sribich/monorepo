import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { URL, fileURLToPath } from "node:url";
import { join } from "node:path";

import { generateDocs } from "@sribich/astro-plugin-docs";

const thisDirectory = fileURLToPath(new URL(".", import.meta.url));
const contentDir = join(thisDirectory, "src/content/docs/generated");

const { integrations, sidebars } = await generateDocs({
    outDir: contentDir,
});

export default defineConfig({
    integrations: [
        // ...integrations,
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
                    items: [
                        // Each item here is one entry in the navigation menu.
                        // { label: "Example Guide", slug: "guides/example2" },
                    ],
                },
                {
                    label: "Packages",
                    items: [
                        // ...sidebars
                    ],
                },
            ],
        }),
    ],
});

/*
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

import { generateDocs } from "@sribich/astro-plugin-docs"
import { URL, fileURLToPath } from 'url';
import { join } from 'path';




export default defineConfig({
    // root: thisDirectory,
    // outDir:
    // site: "https://site.com"
    compressHTML: true,
    integrations: [

        starlight({
            title: "...",
            social: {
                github: "https://github.com/sribich/...",
            },
            sidebar: [
                {
                    label: "Test",
                    autogenerate: { directory: "guides" },
                },
                {
                    label: "Packages",
                    items: [
                        ...sidebars,
                    ]
                }

            ],
        }),
    ],

    // Process images with sharp: https://docs.astro.build/en/guides/assets/#using-sharp
    image: { service: { entrypoint: 'astro/assets/services/sharp' } },
})
 */
