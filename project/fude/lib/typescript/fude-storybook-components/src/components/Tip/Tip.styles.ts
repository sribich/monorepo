import { makeStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"

export const tipStyles = makeStyles({
    slots: create({
        container: {
            padding: 8,
            borderRadius: 12,
            width: "100%",
            height: "fit-content",
        },
        title: {
            fontWeight: 700,
            marginBottom: 8,
            display: "flex",
            gap: 4,
            alignItems: "center",
        },
        content: {},
    }),
    conditions: {},
    variants: {
        color: {
            default: create({
                container: {
                    backgroundColor: "#cdcdcd",
                },
            }),
        },
    },
    defaultVariants: {
        color: "default",
    },
})
