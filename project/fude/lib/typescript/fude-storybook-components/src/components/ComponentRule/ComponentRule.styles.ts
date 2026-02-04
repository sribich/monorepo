import { makeStyles } from "@sribich/fude"
import { create } from "@stylexjs/stylex"

export const componentRuleStyles = makeStyles({
    slots: create({
        container: {
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            backgroundColor: "#cdcdcd",
            height: "200px",
            borderRadius: "16px",
            margin: 0,
            boxShadow: "inset 0 3px 0 #ff0000",
        },
        title: {
            display: "flex",
            alignItems: "center",
            marginTop: 8,
            marginBottom: 4,
            fontWeight: 700,
        },
        icon: {
            height: "20px",
            width: "20px",
            borderRadius: 8,
            marginRight: 4,
            backgroundColor: "#ff0000",
            padding: 2,
            color: "#fff",
        },
        description: {
            fontWeight: 400,
        },
    }),
    conditions: {
        isRecommended: create({
            container: {
                boxShadow: "inset 0 3px 0 #00ff00",
            },
            icon: {
                backgroundColor: "#00ff00",
            },
        }),
    },
    variants: {},
    defaultVariants: {},
})
