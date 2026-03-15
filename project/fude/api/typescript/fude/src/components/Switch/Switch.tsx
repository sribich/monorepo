import { type ComponentRef, type RefObject, useRef } from "react"
import { type AriaSwitchProps, useSwitch, VisuallyHidden } from "react-aria"
import { useToggleState } from "react-stately"

import { useObjectRef } from "../../hooks/useObjectRef"
import { useStyles, type VariantProps } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { mergeRefs } from "../../utils/refs"
import { switchStyles } from "./Switch.styles"

/////////////////////////////////////////////////////////////////////////////////
/// Switch
/////////////////////////////////////////////////////////////////////////////////
export interface SwitchProps extends AriaSwitchProps, VariantProps<typeof switchStyles> {
    ref?: RefObject<HTMLLabelElement>
    /**
     * A custom ref for the input element.
     */
    inputRef?: RefObject<HTMLInputElement>
}

export const Switch = (props: SwitchProps) => {
    const { ref: originalRef, ...restProps } = props

    const ref = useObjectRef(originalRef)
    const inputRef = useObjectRef(
        mergeRefs(restProps.inputRef, useRef<ComponentRef<"input">>(null)),
    )

    const state = useToggleState(restProps)
    const { labelProps, inputProps, isDisabled, isPressed, isReadOnly, isSelected } = useSwitch(
        props,
        state,
        inputRef,
    )

    const { styles } = useStyles(switchStyles, restProps)

    return (
        <label {...mergeProps(labelProps, styles.component())} ref={ref}>
            <VisuallyHidden elementType="span">
                <input ref={inputRef} {...inputProps} />
            </VisuallyHidden>
            <span {...mergeProps(styles.thumbWrapper(isSelected && styles.thumbWrapper.enabled))}>
                <span {...mergeProps(styles.thumb(isSelected && styles.thumb.enabled))}></span>
            </span>
            <span {...mergeProps(styles.label())}>Label</span>
        </label>
    )
}
