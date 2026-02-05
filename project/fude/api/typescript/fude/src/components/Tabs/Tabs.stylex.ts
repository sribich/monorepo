import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"

import { createGenericContext } from "../../hooks/context.js"
import { type CachedStyles, makeStyles } from "../../theme/props.js"

export const tabsStyles = makeStyles({
    slots: create({
        wrapper: {
            display: "flex",
            flexDirection: "column",
            alignItems: "flex-start",
            color: colors.foreground,
        },
        tabList: {
            display: "flex",
            alignItems: "center",
            flexWrap: "nowrap",
            // overflowX: "auto",
            // TODO: "scrollbar-hide",
            padding: spacing["1"],
            // height: "100%",
            backgroundColor: colors.backgroundSecondary,
            overflowX: "visible",
        },
        tabItem: {
            position: "relative",
            zIndex: 0,
            display: "flex",
            flexDirection: "column",
            height: "100%",
            alignItems: "center",
            cursor: "pointer",
            justifyContent: "center",
            paddingLeft: spacing["3"],
            paddingRight: spacing["3"],
            paddingTop: spacing["1"],
            paddingBottom: spacing["1"],
            outline: "none",
            // TODO: "transition-opacity",
            // TODO: ...hocusClasses,
        },
        tabContent: {
            position: "relative",
            zIndex: 10,
            whiteSpace: "nowrap",
            display: "flex",

            alignItems: "center",
            // "text-inherit",
            // "text-neutral-500",
            // "transition-colors",
            // "group-data-[selected=true]:text-neutral-900",
        },
        tabListFlexWrapper: {
            display: "flex",
            width: "100%",
            overflowX: "visible",
        },
        tabListItems: {
            display: "flex",
            flex: "1 1 0%",
            overflowX: "visible",
            gap: spacing["2"],
        },
        tabListAddons: {
            flex: "0 1 auto",
        },
        panel: {
            width: "100%",
            paddingTop: spacing["3"],
            paddingBottom: spacing["3"],
            paddingLeft: spacing["1"],
            paddingRight: spacing["1"],
            // ...hocusClasses
        },
        cursor: {
            position: "absolute",
            zIndex: 0,
            backgroundColor: colors.backgroundSecondaryHover,
        },
        dropIndicator: {
            backgroundColor: "red",
            height: "100%",
            width: "2px",
            marginLeft: "-2px",
        },
    }),
    conditions: {},
    variants: {
        direction: {
            horizontal: create({}),
            vertical: create({
                tabListItems: {
                    flexDirection: "column",
                },
                wrapper: {
                    flexDirection: "row",
                },
            }),
        },
        variant: {
            // underlined
            // pill

            underline: create({
                wrapper: {
                    width: "100%",
                },
                tabList: {
                    width: "100%",

                    backgroundColor: "unset",
                    borderRadius: borderRadius["none"],
                    borderWidth: 0,
                    borderBottomWidth: borderWidth.sm,
                    borderColor: colors.borderLayout,
                    borderStyle: "solid",
                    paddingBottom: 0,
                    // list: "border-default-300 border-b bg-transparent pb-0", // p-0
                },
                tabItem: {
                    borderRadius: borderRadius["none"],
                    ":hover": {
                        borderRadius: borderRadius["none"],
                        // backgroundColor: colors.backgroundHover,
                    },
                },
                cursor: {
                    borderRadius: borderRadius["none"],
                    position: "absolute",
                    left: 0,
                    right: 0,
                    bottom: 0,
                    height: spacing["0.5"],
                    backgroundColor: colors.primary,
                },
                panel: {
                    width: "100%",
                },
            }),
            pill: create({
                tabList: {
                    display: "flex",
                    borderRadius: borderRadius["lg"],
                    boxShadow: boxShadow["md"],
                },
                cursor: {
                    inset: 0,
                    // backgroundColor: colors["backgroundSecondary"],
                    boxShadow: boxShadow["lg"],
                },
            }),
            ghost: create({
                tabList: {
                    backgroundColor: "transparent",
                    // "flex rounded-lg shadow"
                },
                cursor: {
                    inset: 0,
                    // backgroundColor: colors["backgroundSecondary"],
                    boxShadow: boxShadow["lg"],
                },
            }),
            bar: create({
                tabList: {
                    display: "flex",
                    // divide-x
                    // divide-gray-200
                    borderRadius: borderRadius.lg,
                    paddingBottom: 0,
                    boxShadow: boxShadow["md"],
                },
                tabItem: {
                    position: "relative",
                    minWidth: 0,
                    flex: "1 1 0%",
                    padding: spacing["4"],
                    textAlign: "center",
                    // text-sm font-medium hover:bg-gray-50 focus:z-10
                },
                cursor: {
                    position: "absolute",
                    bottom: 0,
                    height: spacing["0.5"],
                    width: "80%",
                },
            }),
        },
        size: {
            sm: create({
                tabItem: {
                    height: spacing["7"],
                    fontSize: fontSize.xs,
                },
            }),
            md: create({
                tabItem: {
                    height: spacing["8"],
                    fontSize: fontSize.sm,
                },
            }),
            lg: create({
                tabItem: {
                    height: spacing["9"],
                    fontSize: fontSize.md,
                },
            }),
        },
        radius: {
            none: create({
                tabList: { borderRadius: borderRadius["none"] },
                tabItem: { borderRadius: borderRadius["none"] },
                cursor: { borderRadius: borderRadius["none"] },
            }),
            sm: create({
                tabList: { borderRadius: borderRadius["md"] },
                tabItem: { borderRadius: borderRadius["sm"] },
                cursor: { borderRadius: borderRadius["sm"] },
            }),
            md: create({
                tabList: { borderRadius: borderRadius["md"] },
                tabItem: { borderRadius: borderRadius["sm"] },
                cursor: { borderRadius: borderRadius["sm"] },
            }),
            lg: create({
                tabList: { borderRadius: borderRadius["lg"] },
                tabItem: { borderRadius: borderRadius["md"] },
                cursor: { borderRadius: borderRadius["md"] },
            }),
            full: create({
                tabList: { borderRadius: borderRadius["full"] },
                tabItem: { borderRadius: borderRadius["full"] },
                cursor: { borderRadius: borderRadius["full"] },
            }),
        },
    },
    defaultVariants: {
        variant: "pill",
        size: "md",
        radius: "none",
        direction: "horizontal",
    },
} as const)

export const [useTabsStyles, TabsStyleProvider] =
    createGenericContext<CachedStyles<typeof tabsStyles>>()
