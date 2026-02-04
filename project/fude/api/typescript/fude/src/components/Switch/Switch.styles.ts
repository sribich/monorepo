import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { absoluteComponentSize } from "@sribich/fude-theme/vars/sizing.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { makeStyles } from "../../theme/props"

export const switchStyles = makeStyles({
    slots: create({
        component: {
            display: "inline-flex",
            alignItems: "center",
            justifyItems: "start",
        },
        thumbWrapper: {
            // include(padding["x-0.5),
            padding: spacing["1"],
            position: "relative",
            display: "inline-flex",
            alignItems: "center",
            justifyContent: "start",
            backgroundColor: "gray",
            borderRadius: borderRadius["full"],
            transitionProperty: "all",
            transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
            transitionDuration: "150ms",
        },
        thumb: {
            zIndex: 2,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            backgroundColor: "white",
            borderRadius: borderRadius["full"],
            transitionProperty: "all",
            transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
            transitionDuration: "150ms",
        },
        label: {},
    }),
    modifiers: {
        enabled: create({
            thumb: {
                // justifyContent: "end",
                marginLeft: spacing["5"],
            },
        }),
    },
    variants: {
        size: {
            sm: create({}),
            md: create({
                thumbWrapper: {
                    // ght: absoluteComponentSize.sm,
                    width: spacing["12"],
                },
                thumb: {
                    height: absoluteComponentSize.xs,
                    width: absoluteComponentSize.xs,
                },
            }),
            lg: create({}),
        },
    },
    defaultVariants: {
        size: "md",
    },
})

/*
<span className="relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-gray-200 transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2">
                <span className="pointer-events-none inline-block h-5 w-5 translate-x-0 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out group-data-[selected=true]:translate-x-5" />
            </span>
 */
