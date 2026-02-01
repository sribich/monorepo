import { defaultMarker } from "@stylexjs/stylex"
import type { RefObject } from "react"
import type { AriaCheckboxProps } from "react-aria"
import { Checkbox as AriaCheckbox } from "react-aria-components"
import { createControlledContext, createGenericContext } from "../../hooks/context"
import { transitionStyles } from "../../theme/atomics/transition"
import { useStyles, type VariantProps } from "../../theme/props"
import type { StyleProps } from "../../utils/props"
import { checkboxStyles } from "./Checkbox.stylex"

//==============================================================================
// Checkbox Utils
//==============================================================================
export interface CheckboxState {
    isIndeterminate: boolean
    isSelected: boolean
}

export const [useCheckboxState, CheckboxState] = createGenericContext<CheckboxState>()
export const [useCheckboxContext, CheckboxContext] = createControlledContext<
    Checkbox.Props,
    HTMLLabelElement
>()

//==============================================================================
// Checkbox
//==============================================================================
export namespace Checkbox {
    export interface Props extends AriaCheckboxProps, VariantProps<typeof checkboxStyles> {
        ref?: RefObject<HTMLLabelElement>
    }
}

export const Checkbox = (_props: Checkbox.Props) => {
    const [props, ref] = useCheckboxContext(_props)

    const { styles } = useStyles(checkboxStyles, props)

    return (
        <AriaCheckbox {...styles.base(defaultMarker())} ref={ref}>
            {({ isIndeterminate, isSelected }) => (
                <CheckboxState value={{ isSelected, isIndeterminate }}>
                    <span
                        {...styles.wrapper(
                            isSelected && styles.wrapper.selected,
                            transitionStyles.color,
                            transitionStyles.movement,
                        )}
                    >
                        <Check
                            {...styles.icon(
                                isSelected && styles.icon.selected,
                                transitionStyles.color,
                                transitionStyles.movement,
                            )}
                        />
                    </span>
                    {props.children && <span {...styles.label()}>{props.children}</span>}
                </CheckboxState>
            )}
        </AriaCheckbox>
    )
}

//==============================================================================
// Check
//==============================================================================
interface CheckProps extends StyleProps {}

const Check = (props: CheckProps) => {
    const { isIndeterminate, isSelected } = useCheckboxState()

    return isIndeterminate ? (
        <svg aria-hidden="true" role="presentation" viewBox="0 0 12 12" {...props}>
            <line x1="2" x2="10" y1="6" y2="6" stroke="currentColor" strokeWidth={2} />
        </svg>
    ) : (
        <svg aria-hidden="true" role="presentation" viewBox="0 0 12 12" {...props}>
            <polyline
                fill="none"
                points="1,7 4,10 10,3"
                stroke="currentColor"
                strokeWidth={2}
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeDasharray={22}
                strokeDashoffset={isSelected ? 44 : 66}
                style={isSelected ? { transition: "stroke-dashoffset 250ms linear 0.2s" } : {}}
            />
        </svg>
    )
}
