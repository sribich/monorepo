import { makeStyles, useStyles, type VariantProps } from "@sribich/fude"
import { create } from "@stylexjs/stylex"
import type { ReactNode } from "react"

//==============================================================================
// Text
//==============================================================================
export namespace Text {
    export interface Props extends VariantProps<typeof textStyles> {
        children: ReactNode
    }
}

export const Text = (props: Text.Props) => {
    const { styles } = useStyles(textStyles, props)

    return <span {...styles.text()}>{props.children}</span>
}

const textStyles = makeStyles({
    slots: {
        text: {},
    },
    variants: {
        language: {
            jp: create({
                text: {
                    fontFamily: "Shippori Mincho",
                    fontSize: 18,
                },
            }),
        },
    },
    defaultVariants: {
        language: "jp",
    },
})
