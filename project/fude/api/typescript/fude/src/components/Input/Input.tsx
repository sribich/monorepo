import type { InputHTMLAttributes, RefObject } from "react"
import { type HoverProps, mergeProps, useFocusRing, useHover } from "react-aria"

import { createControlledContext } from "../../hooks/context"
import { useRenderProps } from "../../hooks/useRenderProps"
import type { StyleRenderProps } from "../../utils/props"

////////////////////////////////////////////////////////////////////////////////
/// Input Utils
////////////////////////////////////////////////////////////////////////////////
export const [useInputContext, InputContext] = createControlledContext<
    InputProps,
    HTMLInputElement
>()

////////////////////////////////////////////////////////////////////////////////
/// InputPrimitive
////////////////////////////////////////////////////////////////////////////////
export interface InputRenderProps {
    /** Whether the input is disabled. */
    isDisabled: boolean
    /** Whether the input is actively being focused by either mouse or keyboard. */
    isFocused: boolean
    /** Whether the input is actively being focused by a keyboard. */
    isFocusVisible: boolean
    /** Whether the input is actively being hovered by a mouse. */
    isHovered: boolean
    /** Whether the input data is invalid. */
    isInvalid: boolean
}

export interface InputProps
    extends Omit<InputHTMLAttributes<HTMLInputElement>, "children" | "className" | "style">,
        Omit<HoverProps, "isDisabled">,
        StyleRenderProps<InputRenderProps> {
    ref?: RefObject<HTMLInputElement>
}

export const Input = (_props: InputProps) => {
    const [props, ref] = useInputContext(_props)

    // TODO: We need to test whether events fire while the input is disabled.
    const { hoverProps, isHovered } = useHover(props)
    const { isFocused, isFocusVisible, focusProps } = useFocusRing({
        isTextInput: true,
        autoFocus: props.autoFocus ?? false,
    })

    const isInvalid = !!props["aria-invalid"] && props["aria-invalid"] !== "false"

    const renderProps = useRenderProps(props, {
        isDisabled: props.disabled ?? false,
        isFocused,
        isFocusVisible,
        isHovered,
        isInvalid,
    })

    return (
        <input
            {...mergeProps(props, focusProps, hoverProps, renderProps)}
            ref={ref}
            data-disabled={props.disabled || undefined}
            data-focused={isFocused || undefined}
            data-focus-visible={isFocusVisible || undefined}
            data-hovered={isHovered || undefined}
            data-invalid={isInvalid || undefined}
        />
    )
}
