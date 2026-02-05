import { defineMain } from "@storybook/react-vite/node"

export default defineMain({
    framework: "@storybook/react-vite",
    stories: [
        "../src/**/*.mdx",
        "../src/**/*.stories.@(js|jsx|ts|tsx)",
        // "../../../api/typescript/fude/src/**/*.mdx",
        "../../../api/typescript/fude/src/**/*.stories.@(js|jsx|ts|tsx)",
    ],
    addons: [
        "@chromatic-com/storybook",
        "@storybook/addon-a11y",
        "@storybook/addon-docs",
        "@storybook/addon-themes",
        "@storybook/addon-vitest",
        "storybook-addon-test-codegen",
    ],
    typescript: {
        // We want aggregated documentation pages, but we do not want autodocs
        // to generate docs for fields which are useless to storybook, to which
        // there are many.
        reactDocgen: false,
    },
})
