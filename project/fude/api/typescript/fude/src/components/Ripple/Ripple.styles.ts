import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"
import type { Ripple } from "./Ripple"

////////////////////////////////////////////////////////////////////////////////
/// Styles
////////////////////////////////////////////////////////////////////////////////
export const rippleStyles = makeStyles({
    slots: {
        ripple: {
            backgroundColor: "currentColor",
            borderRadius: borderRadius.full,
            zIndex: 10,
            position: "absolute",
            pointerEvents: "none",
        },
    },
    modifiers: {
        position: create({
            ripple: (ripple: Ripple.Ripple) => ({
                top: Number(ripple.y),
                left: Number(ripple.x),
                width: Number(ripple.size),
                height: Number(ripple.size),
            }),
        }),
    },
    variants: {},
    defaultVariants: {},
})
