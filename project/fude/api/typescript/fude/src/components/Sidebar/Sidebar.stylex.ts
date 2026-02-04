import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { zIndex } from "@sribich/fude-theme/vars/zindex.stylex"
import { create } from "@stylexjs/stylex"

import { createNewGenericContext } from "../../hooks/context"
import { type CachedStyles, makeStyles } from "../../theme/props"

export const sidebarStyles = makeStyles({
    slots: create({
        // group peer text-sidebar-foreground hidden md:block
        sidebar: {
            height: "100%",
            width: "100%",
        },
        // "relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear",
        sizeKeeper: {
            position: "relative",
            width: "var(--sidebar-width)",
            height: "100%",
            backgroundColor: "transparent",
            // ...transitionStyles.movement, // OLDINCLUDE
        },
        // "hidden md:flex",
        sidebarBody: {
            position: "fixed",
            insetBlock: 0,
            height: "100svh",
            width: "var(--sidebar-width)",
            zIndex: zIndex.infront1,
        },
        sidebarContent: {
            display: "flex",
            height: "100%",
            width: "100%",
            flexDirection: "column",
            backgroundColor: colors.navigationBackground,
        },
        header: {
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["8"],
            padding: newSpacing["8"],
        },
        footer: {
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["8"],
            padding: newSpacing["8"],
        },
        content: {
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["8"],
            flex: "1 1 0%",
            overflow: "auto",
            minHeight: 0,
        },
        group: {
            position: "relative",
            display: "flex",
            flexDirection: "column",
            width: "100%",
            padding: newSpacing["8"],
            minWidth: 0,
        },
        // "text-sidebar-foreground/70 ring-sidebar-ring rounded-md px-2 outline-hidden focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
        groupLabel: {
            display: "flex",
            height: newSpacing["32"],
            alignItems: "center",
            fontSize: fontSize.sm,
            fontWeight: 500,
            color: colors.secondaryForeground,
            userSelect: "none",
            "-webkitUserSelect": "none",
            pointerEvents: "none",
            flexShrink: 0,
            // margin: 0,
        },
        groupContent: {
            width: "100%",
            fontSize: fontSize.sm,
        },
        menu: {
            display: "flex",
            width: "100%",
            minWidth: 0,
            flexDirection: "column",
            gap: newSpacing["4"],
        },
        menuItem: {
            position: "relative",
        },

        //
        //
        //

        // "peer/menu-button  text-sm outline-hidden ring-sidebar-ring hover:bg-sidebar-accent hover:text-sidebar-accent-foreground focus-visible:ring-2 active:bg-sidebar-accent active:text-sidebar-accent-foreground disabled:pointer-events-none disabled:opacity-50 group-has-data-[sidebar=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none aria-disabled:opacity-50 data-[active=true]:bg-sidebar-accent data-[active=true]:font-medium data-[active=true]:text-sidebar-accent-foreground data-[state=open]:hover:bg-sidebar-accent data-[state=open]:hover:text-sidebar-accent-foreground group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0"
        menuButton: {
            display: "flex",
            width: "100%",
            alignItems: "center",
            gap: newSpacing["8"],
            overflow: "hidden",
            borderRadius: borderRadius.md,
            padding: newSpacing["8"],
            textAlign: "left",

            border: 0,
            outline: "none",
            textDecoration: "none",
            color: colors.foreground,
            background: {
                default: "transparent",
                ":hover": colors.backgroundHover,
            },

            ":not(#_) > svg": {
                height: "16px",
                width: "16px",
                flexShrink: 0,
            },
        },
        menuTrigger: {
            display: "flex",
            width: "100%",
            alignItems: "center",
            gap: newSpacing["8"],
            overflow: "hidden",
            borderRadius: borderRadius.md,
            padding: newSpacing["8"],
            textAlign: "left",
            border: 0,
            outline: "none",
            background: {
                default: "transparent",
                ":hover": colors.backgroundHover,
            },
        },
        menuTriggerIcon: {},
        menuTriggerContent: {
            flex: "1 1 auto",
        },
        menuTriggerChevron: {
            transition: "transform 200ms cubic-bezier(0.4, 0, 0.2, 1)",
            color: colors.secondaryForeground,
            height: "14px",
            flex: "0 1 auto",
        },
        rail: {
            position: "absolute",
            border: 0,
            top: 0,
            bottom: 0,
            width: newSpacing["16"],
            display: "flex",
            zIndex: 20,
            backgroundColor: "#0000",
            transform: "translateX(calc(1/2 * 100% * -1))",
            ":hover::after": {
                backgroundColor: colors.borderLayoutHover,
            },
            "::after": {
                content: "",
                position: "absolute",
                top: 0,
                bottom: 0,
                left: "calc(1/2 * 100%)",
                width: "2px",
            },
        },
    }),
    conditions: {
        collapsed: {
            true: create({}),
            false: {},
        },
    },
    modifiers: {
        pathSelected: create({
            menuButton: {
                backgroundColor: colors.primarySelected,
            },
        }),
        menuOpen: create({
            menuTriggerChevron: {
                transform: "rotate(90deg)",
            },
        }),
    },
    variants: {
        collapsible: {
            none: create({}),
            icon: create({}),
            full: create({}),
        },
        side: {
            left: create({
                sidebarBody: {
                    left: 0,
                },
                rail: {
                    right: `calc(${newSpacing["16"]} * -1)`,
                    cursor: "w-resize",
                },
            }),
            right: create({
                sizeKeeper: {
                    transform: "rotate(180)",
                },
                sidebarBody: {
                    right: 0,
                },
                rail: {
                    left: 0,
                    cursor: "e-resize",
                },
            }),
        },
        size: {
            xs: create({
                menuButton: {
                    // ...componentSize.xs, // OLDINCLUDE
                },
                menuTrigger: {
                    // ...componentSize.xs,
                },
            }),
            sm: create({
                menuButton: {
                    // ...componentSize.sm,
                },
                menuTrigger: {
                    // ...componentSize.sm,
                },
            }),
            md: create({
                menuButton: {
                    // ...componentSize.md,
                },
                menuTrigger: {
                    // ...componentSize.md,
                },
            }),
            lg: create({
                menuButton: {
                    // ...componentSize.lg,
                },
                menuTrigger: {
                    // ...componentSize.lg,
                },
            }),
        },
        variant: {
            inline: create({}),
            island: create({
                sizeKeeper: {},
                sidebarBody: {
                    padding: newSpacing["8"],
                    // boxSizing: "border-box",
                },
                sidebarContent: {
                    borderRadius: borderRadius.lg,
                    borderWidth: borderWidth.sm,
                    border: "1px",
                    borderStyle: "solid",
                    borderColor: colors.borderLayout,
                    boxShadow: boxShadow.md,
                    // boxSizing: "border-box",
                },
            }),
        },
    },
    defaultVariants: {
        collapsible: "none",
        side: "left",
        size: "sm",
        variant: "inline",
    },
    compounds: [
        //======================================================================
        // Left & Right
        //======================================================================
        {
            variants: {
                side: "left",
            },
            modify: {
                variant: {
                    inline: create({
                        sidebarBody: {
                            borderRightWidth: borderWidth.sm,
                            borderRightColor: colors.borderLayout,
                            borderRightStyle: "solid",
                        },
                    }),
                },
            },
        },
        {
            variants: {
                side: "right",
            },
            modify: {
                variant: {
                    inline: create({
                        sidebarBody: {
                            borderLeftWidth: borderWidth.sm,
                            borderLeftColor: colors.borderLayout,
                            borderRightStyle: "solid",
                        },
                    }),
                },
            },
        },
        {
            variants: {
                side: "left",
            },
            conditions: {
                collapsed: true,
            },
            modify: {
                collapsible: {
                    full: create({
                        sidebarBody: {
                            left: "calc(var(--sidebar-width) * -1)",
                        },
                        rail: {
                            right: `calc(${newSpacing["8"]} * -1)`,
                        },
                    }),
                },
            },
        },
        {
            variants: {
                side: "right",
            },
            conditions: {
                collapsed: true,
            },
            modify: {
                collapsible: {
                    full: create({
                        sidebarBody: {
                            right: "calc(var(--sidebar-width) * -1)",
                        },
                        rail: {
                            left: `calc(${newSpacing["8"]} * -1)`,
                        },
                    }),
                },
            },
        },
        //
        //
        //
        {
            variants: {
                variant: "island",
            },
            conditions: {
                collapsed: true,
            },
            modify: {
                collapsible: {
                    icon: create({
                        sizeKeeper: {
                            width: `calc(var(--sidebar-width-icon) + ${newSpacing["16"]})`,
                        },
                        sidebarBody: {
                            width: `calc(var(--sidebar-width-icon) + ${newSpacing["16"]} + 2px)`,
                        },
                    }),
                },
            },
        },
        {
            variants: {
                variant: "inline",
            },
            conditions: {
                collapsed: true,
            },
            modify: {
                collapsible: {
                    icon: create({
                        sizeKeeper: {
                            width: `var(--sidebar-width-icon)`,
                        },
                        sidebarBody: {
                            width: "var(--sidebar-width-icon)",
                        },
                    }),
                },
            },
        },
        //
        //
        //
        {
            conditions: {
                collapsed: true,
            },
            modify: {
                collapsible: {
                    full: create({
                        sizeKeeper: {
                            width: 0,
                        },
                        rail: {
                            transform: "translateX(0)",
                            "::after": {
                                left: "100%",
                            },
                            ":hover": {
                                backgroundColor: colors.backgroundSecondary,
                            },
                        },
                    }),
                    icon: create({
                        content: {
                            overflow: "hidden", // Prevent scrolling
                        },
                        groupLabel: {
                            marginTop: `calc(${newSpacing["32"]} * -1)`,
                            opacity: 0,
                        },
                    }),
                },
            },
        },
    ],
})

export const SidebarStyles = createNewGenericContext<CachedStyles<typeof sidebarStyles>>()
