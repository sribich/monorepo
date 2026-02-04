import { newSpacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { create } from "@stylexjs/stylex"
import { createContext } from "react"
import { Form as AriaForm, type FormProps as AriaFormProps } from "react-aria-components"
import { makeStyles, useStyles, type VariantProps } from "../../theme/props"

//==============================================================================
// FormContext
//==============================================================================
export interface FormContext {
    /**
     * The default label placement for form items.
     *
     * @default "inside"
     */
    labelPlacement: "inside" | "outside" | "outside-left"
    /**
     * The size to use for form components.
     *
     * @default "md"
     */
    size?: "sm" | "md" | "lg"
}

export const FormContext = createContext<FormContext | undefined>(undefined)

//==============================================================================
// Form
//==============================================================================
export namespace Form {
    export interface Props extends AriaFormProps, FormContext, VariantProps<typeof formStyle> {}
}

export const Form = (props: Form.Props) => {
    const { children, labelPlacement = "inside", size = "md", ...restProps } = props

    const { styles } = useStyles(formStyle, {})

    return (
        <AriaForm {...restProps} {...styles.form()}>
            <FormContext value={{ labelPlacement, size }}>{children}</FormContext>
        </AriaForm>
    )
}

//==============================================================================
// Style
//==============================================================================
const formStyle = makeStyles({
    slots: create({
        form: {
            display: "flex",
            flexDirection: "column",
            gap: newSpacing["8"],
            alignItems: "start",
        },
    }),
    modifiers: {},
    conditions: {},
    variants: {},
    defaultVariants: {},
})
