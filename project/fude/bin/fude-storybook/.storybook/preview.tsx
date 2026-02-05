import addonA11y from "@storybook/addon-a11y"
import addonDocs from "@storybook/addon-docs"
import { type Decorator, definePreview } from "@storybook/react-vite"

import "./preview.css"
import "@sribich/fude/reset.css"
import "../src/styles.css"
import "../src/tailwind.css"

import { darkTheme, lightTheme } from "@sribich/fude-theme"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { create, props } from "@stylexjs/stylex"

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
        docs: {
            toc: true,
        },
    },
    decorators: [withThemeDecorator],
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
    },
    tags: ["autodocs"],
})

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

import { DocsContainer, DocsPage, Unstyled } from "@storybook/blocks"

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
}
*/
