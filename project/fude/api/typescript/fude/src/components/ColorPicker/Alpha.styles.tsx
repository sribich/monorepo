import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"

export const alphaStyles = makeStyles({
    slots: create({
        alphaContainer: {
            position: "absolute",
            inset: 0,

            // overflow: "hidden",
            boxShadow: boxShadow.inset,
        }, // className="absolute inset-0 bg-blue-900 z-10 h-full"
        alphaSlider: {
            position: "absolute",
            inset: 0,
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
