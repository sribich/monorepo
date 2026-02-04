import { makeStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"

export const componentNameStyles = makeStyles({
    slots: create({
        container: {
            width: "100%",
            padding: "40px 24px",
            marginTop: 0,
            marginBottom: 32,
            fontSize: "40px",
            fontWeight: 600,
            background: `url('/component.webp')`,
            backgroundSize: "cover",
            backgroundPosition: "center",
        },
        content: {
            textShadow: "white 0px 0px 10px",
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})
