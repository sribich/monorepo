import { type ElementRef, useRef, type MutableRefObject, type RefObject } from "react"
import { type AriaSwitchProps, VisuallyHidden, useSwitch } from "react-aria"
import { useToggleState } from "react-stately"
import { useStyles, type VariantProps } from "../../theme/props"
import { switchStyles } from "./Switch.styles"
import { mergeProps } from "../../utils/mergeProps"
import { useObjectRef } from "../../hooks/useObjectRef"
import { mergeRefs } from "../../utils/refs"

/////////////////////////////////////////////////////////////////////////////////
/// Switch
/////////////////////////////////////////////////////////////////////////////////
export interface SwitchProps extends AriaSwitchProps, VariantProps<typeof switchStyles> {
    ref?: RefObject<HTMLLabelElement>
    /**
     * A custom ref for the input element.
     */
    inputRef?: MutableRefObject<HTMLInputElement>
}

export const Switch = (props: SwitchProps) => {
    const ref = useObjectRef(props.ref)
    const inputRef = useObjectRef(mergeRefs(props.inputRef, useRef<ElementRef<"input">>(null)))

    const state = useToggleState(props)
    const { labelProps, inputProps, isDisabled, isPressed, isReadOnly, isSelected } = useSwitch(
        props,
        state,
        inputRef,
    )

    const { styles } = useStyles(switchStyles, props)

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
