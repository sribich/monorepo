import addonA11y from "@storybook/addon-a11y"
import addonDocs from "@storybook/addon-docs"
import { definePreview } from "@storybook/react-vite"

import "./preview.css"
import "@sribich/fude/reset.css"
import "../src/styles.css"
import "../src/tailwind.css"

export default definePreview({
    addons: [addonA11y(), addonDocs()],
    parameters: {
        a11y: {
            options: { xpath: true },
        },
        docs: {
            toc: true,
        },
    },
    tags: ["autodocs"],
})

/*

*/

/*
import {
    ComponentName,
    ComponentRule,
    ComponentRules,
    PropsTable,
    SectionName,
    Tip,
    Title,
    UnstyledList,
    UnstyledListItem,
    UsageGuidelines,
} from "@sribich/fude-storybook"

import { DecoratorHelpers } from "@storybook/addon-themes"
import { DocsContainer, DocsPage, Unstyled } from "@storybook/blocks"

import { create, props } from "@stylexjs/stylex"
import type { ReactNode } from "react"
import type React from "react"
import { Link, RelatedComponents } from "vibe-storybook-components"
// import "vibe-storybook-components/dist/index.css"

// import "@sribich/fude-storybook/styles.css"
import "./preview.css"
import "@sribich/fude/styles.css"
import "../src/styles.css"

// We need to import our storybook components like this so that
// the imports are not hoisted so that we can inject the stylex
// dev-runtime.
// import { colors } from "../../../lib/typescript/ui/src/theme/vars/colors.stylex"
// import { darkTheme } from "../../../lib/typescript/ui/src/theme/themes"
// import { colors } from "@sribich/fude-theme/vars/color.stylex"
import { darkTheme } from "@sribich/fude-theme"

const themeStyles = create({
    maxHeight: {
        height: "100%",
    },
    flexWrapper: {
        height: "100%",
        display: "flex",
        justifyContent: "center",
        // backgroundColor: colors.background,
    },
    themeWrapper: {
        // maxWidth: "1000px",
        width: "100%",
        height: "100%",
        // color: colors.foreground,
        // backgroundColor: colors.background,

        // "--ui-background": colors.background,
        // "--ui-foreground": colors.foreground,
    },
})

const { initializeThemeState, pluckThemeFromContext } = DecoratorHelpers

const RelatedComponentsDecorator = ({ componentsNames, linkTarget }) => {
    return (
        <RelatedComponents
            componentsNames={componentsNames}
            linkTarget={linkTarget}
            descriptionComponentsMap={
                new Map([
                    // ["button", <Button />]
                ])
            }
        />
    )
}

let theme = "light"

const withThemeDecorator = <T extends Record<string, boolean>>(args: {
    themes: T
    defaultTheme: keyof T
}) => {
    const defaultTheme = String(args.defaultTheme)

    initializeThemeState(Object.keys(args.themes), defaultTheme)

    const decorator = (Story: React.FC, context: never) => {
        const selectedTheme = pluckThemeFromContext(context)

        const selected = selectedTheme || defaultTheme
        theme = selected

        console.log(theme, darkTheme)

        const { className } = props(themeStyles.maxHeight, selected === "dark" && darkTheme)

        return (
            <div className={`${className}`}>
                <div {...props(themeStyles.themeWrapper)}>
                    <Story />
                </div>
            </div>
        )
    }

    decorator.displayName = "ThemeDecorator"

    return decorator
}

export default {
    parameters: {
        actions: { argTypesRegex: "^on.*" },
        layout: "fullscreen",
        docs: {
            story: {
                inline: true,
            },
            container: (_props: { children: ReactNode; context: never }) => {
                return (
                    <DocsContainer context={_props.context}>
                        <Unstyled>
                            <div {...props(themeStyles.maxHeight, theme === "dark" && darkTheme)}>
                                <div {...props(themeStyles.flexWrapper)}>
                                    <div {...props(themeStyles.themeWrapper)}>
                                        {_props.children}
                                    </div>
                                </div>
                            </div>
                        </Unstyled>
                    </DocsContainer>
                )
            },
            page: DocsPage,
            components: {
                ArgsTable: PropsTable,
                Controls: PropsTable,
                PropsTable,

                h1: ComponentName,
                h2: SectionName,
                h3: Title,
                li: UnstyledListItem,
                ul: UnstyledList,
                ComponentName,
                ComponentRule,
                ComponentRules,
                Tip,
                UnstyledList,
                UnstyledListItem,
                UsageGuidelines,

                Link,
                RelatedComponents: RelatedComponentsDecorator,
                /*
                p: Paragraph,
                AlphaWarning,
                DeprecatedWarning,
                SectionName,
                FunctionArguments,
                FunctionArgument,
                RelatedComponent,
                Frame,
                * /
            },
        },
        options: {
            storySort: {
                order: [
                    "Welcome",
                    "Foundations",
                    "Layout",
                    "Navigation",
                    "Typography",
                    "Interactions",
                    "Data Entry",
                    "Data Display",
                    "Media",
                    "Overlays",
                    "Feedback",
                    "Hooks",
                    "Utils",
                    "*",
                ],
            },
        },
    },
    decorators: [
        (Story: React.FC) => {
            return (
                <div className="w-full p-8">
                    <Story />
                </div>
            )
        },
        withThemeDecorator({
            themes: {
                light: true,
                dark: true,
            },
            defaultTheme: "light",
        }),
    ],
}
*/
