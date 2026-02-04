import { parentMarker } from "@sribich/fude-theme/markers.stylex"
import { type RefObject, use, useRef, useState } from "react"
import { mergeProps, useFocusWithin } from "react-aria"
import {
    TextField as AriaTextField,
    type TextFieldProps as AriaTextFieldProps,
    Input,
    Label,
} from "react-aria-components"

import { transitionStyles } from "../../theme/atomics/transition"
import { useStyles, type VariantProps } from "../../theme/props"
import { FormContext } from "../Form/Form"
import { textFieldStyles } from "./TextField.stylex"

//==============================================================================
// TextField
//==============================================================================
export interface TextFieldProps extends AriaTextFieldProps, VariantProps<typeof textFieldStyles> {
    ref?: RefObject<HTMLDivElement>
    label?: string

    placeholder?: string
}

export const TextField = (props: TextFieldProps) => {
    const formContext = use(FormContext)

    const isLabelInteractive = true

    // const styles = textFieldVariants({ ...props, isLabelInteractive })

    const inputRef = useRef<HTMLInputElement>(null)
    const [isFocusWithin, setFocusWithin] = useState(false)

    const { focusWithinProps } = useFocusWithin({
        onFocusWithinChange: setFocusWithin,
    })

    const hasContent = !!inputRef.current?.value || !!props.placeholder || isFocusWithin

    const {
        styles,
        values: { labelPlacement },
    } = useStyles(textFieldStyles, {
        ...formContext,
        ...props,
        hasContent: !!hasContent,
        focused: isFocusWithin,
    })

    const isLabelOutside = labelPlacement === "outside-left" || labelPlacement === "outside-top"
    const labelComponent = (
        <Label {...styles.label(hasContent && styles, transitionStyles.movement)}>
            {props.label}
        </Label>
    )

    return (
        <AriaTextField
            {...mergeProps(props, styles.textField(parentMarker), focusWithinProps)}
            data-focused={isFocusWithin}
        >
            {isLabelOutside && labelComponent}
            <div {...styles.inputGroup()}>
                {isLabelOutside && labelComponent}
                {!isLabelOutside && labelComponent}
                <div {...styles.inputInner()}>
                    <Input {...styles.input()} ref={inputRef} />
                </div>
            </div>
            {/*helper*/}
        </AriaTextField>
    )
}

//==============================================================================
// Styles
//==============================================================================
