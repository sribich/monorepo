import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const checkerboardStyles = makeStyles({
    slots: create({
        checkerboard: {
            position: "relative",
            height: "100%",
            width: "100%",
        },
    }),
    modifiers: {
        background: create({
            checkerboard: (image: string) => ({
                background: image,
            }),
        }),
    },
    variants: {
        radius: {
            none: create({
                checkerboard: {
                    borderRadius: borderRadius["none"],
                },
            }),
            sm: create({
                checkerboard: {
                    borderRadius: borderRadius["sm"],
                },
            }),
            md: create({
                checkerboard: {
                    borderRadius: borderRadius["md"],
                },
            }),
            lg: create({
                checkerboard: {
                    borderRadius: borderRadius["lg"],
                },
            }),
            full: create({
                checkerboard: {
                    borderRadius: borderRadius["full"],
                },
            }),
        },
    },
    defaultVariants: {
        radius: "none",
    },
})
