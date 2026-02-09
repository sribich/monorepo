import {
    ComponentName,
    ComponentRule,
    ComponentRules,
    SectionName,
    Tip,
    Title,
    UnstyledList,
    UnstyledListItem,
    UsageGuidelines,
} from "@sribich/fude-storybook-components"
import { darkTheme, lightTheme } from "@sribich/fude-theme"
import { fonts } from "@sribich/fude-theme/vars/fonts.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import addonA11y from "@storybook/addon-a11y"
import addonDocs from "@storybook/addon-docs"
import {
    Canvas,
    Controls,
    DocsContainer,
    DocsPage,
    Meta,
    Unstyled,
} from "@storybook/addon-docs/blocks"
import { type Decorator, definePreview } from "@storybook/react-vite"
import { create, createTheme, props } from "@stylexjs/stylex"
import { MDXBadges } from "storybook-addon-tag-badges/manager-helpers"

import "./preview.css"
import "@sribich/fude/reset.css"
import "../src/styles.css"
import "../src/tailwind.css"

import { demoModeLoader } from "./demo-mode"

const fontTheme = createTheme(fonts, {
    display: "Figtree",
    english: "Inter",
})

const themeStyles = create({
    themeWrapper: {
        height: "100%",
        display: "flex",
    },
    themeElement: {
        flex: 1,
        height: "100%",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        overflow: "auto",
        padding: "1rem",
        backgroundColor: colors.background,
    },
    docsWrapper: {
        height: "100%",
        padding: "80px 0 64px 0",
        color: colors.foreground,
        backgroundColor: colors.background,
        display: "flex",
        justifyContent: "center",
    },
    docs: {
        maxWidth: "1000px",
        minWidth: "0px",
        width: "100%",
        height: "100%",
    },
})

const withThemeDecorator: Decorator = (Story, context) => {
    const theme = context.parameters["theme"] || context.globals["theme"] || "light"

    if (theme === "side-by-side") {
        return (
            <div {...props(themeStyles.themeWrapper)}>
                <div {...props(themeStyles.themeElement, lightTheme)}>
                    <Story />
                </div>
                <div {...props(themeStyles.themeElement, darkTheme)}>
                    <Story />
                </div>
            </div>
        )
    }

    return (
        <div {...props(themeStyles.themeElement, theme === "dark" && darkTheme)}>
            <Story />
        </div>
    )
}

export default definePreview({
    addons: [addonA11y(), addonDocs()],
    parameters: {
        a11y: {
            options: { xpath: true },
        },
        layout: "fullscreen",
        docs: {
            canvas: {
                layout: "fullscreen",
            },
            container: ({ children, context }) => {
                const theme = context.store.userGlobals.globals.theme || "light"

                return (
                    <DocsContainer context={context}>
                        <Unstyled>
                            <div
                                {...props(
                                    themeStyles.docsWrapper,
                                    theme === "dark" && darkTheme,
                                    fontTheme,
                                )}
                            >
                                <div {...props(themeStyles.docs)}>{children}</div>
                            </div>
                        </Unstyled>
                    </DocsContainer>
                )
            },
            page: DocsPage,
            components: {
                Canvas, //
                ComponentName, //
                ComponentRule,
                ComponentRules,
                Controls, //
                Link: () => null,
                Meta,
                h1: ComponentName, //
                h2: SectionName, //
                h3: Title,
                li: UnstyledListItem,
                ul: UnstyledList,
                Tip,
                UnstyledList,
                UnstyledListItem,
                UsageGuidelines,
                RelatedComponents: () => null,
                MDXBadges,
                /*
                RelatedComponents: ({ componentNames, linkTarget }) => (
                    <RelatedComponents
                        componentsNames={componentsNames}
                        linkTarget={linkTarget}
                        descriptionComponentsMap={
                            new Map([
                                // ["button", <Button />]
                            ])
                        }
                    />
                ),
                /*
                p: Paragraph,
                AlphaWarning,
                DeprecatedWarning,
                SectionName,

                FunctionArguments,
                FunctionArgument,
                RelatedComponent,
                Frame,
                StorybookLink,
                */
            },
        },
    },
    decorators: [withThemeDecorator],
    loaders: [demoModeLoader],
    globalTypes: {
        theme: {
            name: "Theme",
            description: "Theme for the components",
            defaultValue: "light",
            toolbar: {
                icon: "circlehollow",
                items: [
                    { value: "light", icon: "sun", title: "light" },
                    { value: "dark", icon: "moon", title: "dark" },
                    { value: "side-by-side", icon: "sidebar", title: "side by side" },
                ],
            },
        },
        demoMode: {
            name: "Demo Mode",
            description: "Slows actions when running interactions to mimic user navigation",
            defaultValue: "off",
            toolbar: {
                icon: "circlehollow",
                items: [
                    { value: "on", icon: "sun", title: "On" },
                    { value: "off", icon: "moon", title: "Off" },
                ],
            },
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
    tags: ["autodocs"],
})
