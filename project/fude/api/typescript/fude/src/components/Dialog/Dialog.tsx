import { create } from "@stylexjs/stylex"
import { mergeProps } from "react-aria"
import {
    type DialogProps,
    Dialog as RacDialog,
    DialogTrigger as RacDialogTrigger,
} from "react-aria-components"
import { makeStyles, useStyles, type VariantProps } from "../../theme/props"

//==============================================================================
// Styles
//==============================================================================
export const dialogStyles = makeStyles({
    slots: create({
        dialog: {
            // backgroundColor: colors.background,
            // boxShadow: boxShadow.sm,
        },
    }),
    conditions: {},
    variants: {},
    defaultVariants: {},
})

//==============================================================================
// DialogTrigger
//==============================================================================
export const DialogTrigger = RacDialogTrigger

//==============================================================================
// Dialog
//==============================================================================
export namespace Dialog {
    export interface Props extends DialogProps, VariantProps<typeof dialogStyles> {}
}

export const Dialog = (props: Dialog.Props) => {
    const { styles } = useStyles(dialogStyles, props)

    return <RacDialog {...mergeProps(props, styles.dialog())} />
}
