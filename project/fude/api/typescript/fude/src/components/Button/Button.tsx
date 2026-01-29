import { filterDOMProps, mergeProps } from "@react-aria/utils"
import { parentMarker } from "@sribich/fude-theme/vars/markers.stylex"
import { LoaderCircle } from "lucide-react"
import { type ReactElement, type ReactNode, type RefObject, use } from "react"
import { useButton, useFocusRing, useHover } from "react-aria"
import {
    Button as AriaButton,
    type ButtonProps as AriaButtonProps,
    useRenderProps,
} from "react-aria-components"
import { createNewControlledContext } from "../../hooks/context.js"
import { useObjectRef } from "../../hooks/useObjectRef.js"
import { stylexColorVariants } from "../../theme/atomics/color.js"
import { componentSize } from "../../theme/atomics/componentSize.js"
import { focusStyles } from "../../theme/atomics/focus.js"
import { transitionStyles } from "../../theme/atomics/transition.js"
import { useStyles, type VariantProps } from "../../theme/props.js"
import { Delegate } from "../Delegate/Delegate.js"
import { FormContext } from "../Form/Form.js"
import { useRipple } from "../Ripple/Ripple.hook.js"
import { Ripple } from "../Ripple/Ripple.js"
import { buttonGroupStyles, buttonStyles } from "./Button.styles.js"

//==============================================================================
// ButtonGroup
//==============================================================================
const ButtonGroupContext = createNewControlledContext<Button.Props, Button.RefType>()

export namespace ButtonGroup {
    export interface Props extends VariantProps<typeof buttonStyles> {
        children: ReactNode
    }
}

export const ButtonGroup = (props: ButtonGroup.Props) => {
    const { children, ...restProps } = props

    const { styles } = useStyles(buttonGroupStyles, {})

    return (
        <ButtonGroupContext value={restProps}>
            <div {...mergeProps(styles.container(), parentMarker)}>{props.children}</div>
        </ButtonGroupContext>
    )
}

//==============================================================================
// Button
//==============================================================================
export namespace Button {
    export type RefType = HTMLButtonElement

    export interface Props
        extends AriaButtonProps,
        Omit<VariantProps<typeof buttonStyles>, "inGroup"> {
        ref?: RefObject<HTMLButtonElement>
        isLoading?: boolean

        startContent?: ReactNode
        endContent?: ReactNode
    }
}

export const Button = (_props: Button.Props): ReactElement => {
    const [props, ref] = ButtonGroupContext.useContext(_props)

    const formContext = use(FormContext)

    const { rippleProps, ripples, clearRipple } = useRipple(false)
    const isDisabled = !!(props.isDisabled || props.isLoading)

    const {
        styles,
        values: { variant, color, size },
    } = useStyles(buttonStyles, {
        ...formContext,
        ...props,
        inGroup: ButtonGroupContext.isProvided(),
    })

    const styleProps = styles.button(
        transitionStyles.color,
        focusStyles,
        componentSize[size],
        stylexColorVariants[variant][color],
    )

    if (typeof props.children === "function") {
        return (
            <AriaButton {...mergeProps(props, rippleProps, styleProps)} isDisabled={isDisabled}>
                {props.children}
            </AriaButton>
        )
    }

    return (
        <AriaButton {...mergeProps(props, rippleProps, styleProps)} isDisabled={isDisabled} ref={ref}>
            {props.isLoading && <LoaderCircle {...styles.spin()} />}
            {!!props.startContent && props.startContent}
            {props.children}
            {!!props.endContent && props.endContent}
            <Ripple ripples={ripples} clearRipple={clearRipple} />
        </AriaButton>
    )
}

//==============================================================================
// DelegateButton
//==============================================================================
export const DelegateButton = (props: Button.Props) => {
    const ref = useObjectRef(props.ref)
    const { buttonProps, isPressed } = useButton(props, ref)

    const { focusProps, isFocused, isFocusVisible } = useFocusRing(props)
    const { hoverProps, isHovered } = useHover(props)

    const renderProps = useRenderProps({
        ...props,
        values: {
            isDisabled: props.isDisabled || false,
            isFocused,
            isFocusVisible,
            isHovered,
            isPressed,
            isPending: false,
        },
        defaultClassName: "",
    })

    const {
        styles,
        values: { variant, color },
    } = useStyles(buttonStyles, props)

    const styleProps = styles.button(
        transitionStyles.color,
        focusStyles,
        stylexColorVariants[variant][color],
    )

    return (
        <Delegate
            {...mergeProps(
                filterDOMProps(props),
                buttonProps,
                focusProps,
                hoverProps,
                styleProps,
                renderProps,
            )}
            ref={ref}
            slot={props.slot || undefined}
            data-disabled={props.isDisabled || undefined}
            data-focused={isFocused || undefined}
            data-focus-visible={isFocusVisible || undefined}
            data-hovered={isHovered || undefined}
            data-pressed={isPressed || undefined}
        >
            {renderProps.children}
        </Delegate>
    )
}
