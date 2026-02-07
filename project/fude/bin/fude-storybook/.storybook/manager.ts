import { addons } from "storybook/manager-api"
import { create } from "storybook/theming/create"
import type { TagBadgeParameters } from "storybook-addon-tag-badges/manager-helpers"

const badgeDisplay = {
    sidebar: [
        {
            type: "component" as const,
            skipInherited: true,
        },
    ],
    toolbar: true,
    mdx: true,
}

addons.setConfig({
    theme: create({
        base: "dark",
        brandTitle: "@sribich/fude",
    }),
    tagBadges: [
        {
            tags: "new",
            badge: {
                text: "New",
                style: {
                    backgroundColor: "#84cc16",
                    color: "#000",
                },
                tooltip: "This component was recently stabilized",
            },
            display: badgeDisplay,
        },
        {
            tags: "updated",
            badge: {
                text: "Updated",
                style: {
                    backgroundColor: "#0ea5e9",
                    color: "#000",
                },
                tooltip: "This component is stable and was recently updated",
            },
            display: badgeDisplay,
        },
        {
            tags: "beta",
            badge: {
                text: "Beta",
                style: {
                    backgroundColor: "#a855f7",
                    color: "#fff",
                },
                tooltip: "This component is being tested for stability. The component is unlikely to change significantly prior to stabilization",
            },
            display: badgeDisplay,
        },
        {
            tags: "alpha",
            badge: {
                text: "Alpha",
                style: {
                    backgroundColor: "#d946ef",
                    color: "#fff",
                },
                tooltip: "This component is still being developed and is expected to be stabilized. There may be significant changes in any given update and this component is not recommended for production use",
            },
            display: badgeDisplay,
        },
        {
            tags: "experimental",
            badge: {
                text: "Experimental",
                style: {
                    backgroundColor: "#ec4899",
                    color: "#fff",
                },
                tooltip: "This component is still being developed and may not be stabilized. There may be significant changes in any given update and this component is not recommended for production use",
            },
            display: badgeDisplay,
        },
        {
            tags: "deprecated",
            badge: {
                text: "Deprecated",
                style: {
                    backgroundColor: "#ef4444",
                    color: "#fff",
                },
                tooltip: "This component is deprecated and will be removed in the next major release",
            },
            display: badgeDisplay,
        },
        {
            tags: "risky",
            badge: {
                text: "Risky",
                style: {
                    backgroundColor: "#f97316",
                    color: "#fff",
                },
                tooltip: "This component is difficult to implement correctly",
            },
            display: badgeDisplay,
        },
    ] satisfies TagBadgeParameters,
})
