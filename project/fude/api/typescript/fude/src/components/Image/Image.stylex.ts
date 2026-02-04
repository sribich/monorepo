import { zIndex } from "@sribich/fude-theme/vars/zindex.stylex"
import { create } from "@stylexjs/stylex"

import { makeStyles } from "../../theme/props"

export const imageStyles = makeStyles({
    slots: create({
        wrapper: {
            position: "relative",
        },
        image: {
            position: "relative",
            zIndex: zIndex.infront1,
        },
        zoom: {
            position: "relative",
            overflow: "hidden",
        },
        blur: {
            position: "absolute",
            zIndex: zIndex.behind1,
            inset: 0,
            height: "100%",
            width: "100%",
            objectFit: "cover",
            filter: "blur(12px), saturate(150%)",
            opacity: "30%",
            scale: "105%",
        },
    }),
    conditions: {
        zoom: create({
            image: {
                objectFit: "cover",
                transform: "1",
                ":hover": {
                    scale: "125%",
                },
            },
        }),
    },
    variants: {},
    defaultVariants: {},
})
