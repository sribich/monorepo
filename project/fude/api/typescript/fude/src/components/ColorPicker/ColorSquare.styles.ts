import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { boxShadow } from "@sribich/fude-theme/vars/boxShadow.stylex"

export const colorSquareStyles = makeStyles({
    slots: create({
        colorSquareContainer: {
            position: "relative",
            height: "100%",
            width: "100%",
            overflow: "hidden",
            boxShadow: boxShadow.inset,
        },
        colorSquare: {
            position: "absolute",
            inset: 0,
        },
    }),
    variants: {
        radius: {
            none: create({
                colorSquareContainer: {
                    borderRadius: borderRadius["none"],
                },
            }),
            sm: create({
                colorSquareContainer: {
                    borderRadius: borderRadius["sm"],
                },
            }),
            md: create({
                colorSquareContainer: {
                    borderRadius: borderRadius["md"],
                },
            }),
            lg: create({
                colorSquareContainer: {
                    borderRadius: borderRadius["lg"],
                },
            }),
            full: create({
                colorSquareContainer: {
                    borderRadius: borderRadius["full"],
                },
            }),
        },
    },
    defaultVariants: {
        radius: "none",
    },
})
