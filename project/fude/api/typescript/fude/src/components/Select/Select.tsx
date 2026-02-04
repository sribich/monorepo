import { ChevronDown } from "lucide-react"
import type { ReactNode, RefObject } from "react"
import { Select as AriaSelect, Popover, type SelectProps, SelectValue } from "react-aria-components"
import type { Key } from "react-stately"
import { useStyles, type VariantProps } from "../../theme/props"
import { Button } from "../Button/Button"
import { ListBox, ListBoxItem } from "../ListBox/ListBox"
import { selectStyles } from "./Select.stylex"
import { mergeProps } from "../../utils/mergeProps"

//==============================================================================
// Select
//==============================================================================
export namespace Select {
    export interface Props<T extends object>
        extends Omit<SelectProps<T>, "children">,
            VariantProps<typeof selectStyles> {
        selectRef: RefObject<HTMLDivElement>
        label: string

        items?: Iterable<T>
        children?: ReactNode | ((item: T) => ReactNode)
    }
}

export const Select = <T extends object>(props: Select.Props<T>) => {
    const { label, children, items, ...restProps } = props

    const { styles } = useStyles(selectStyles, props)

    /*
    const label = (
        <TypographyText {...mergeProps(labelProps, styles.label())}>{props.label}</TypographyText>
    )
    */

    return (
        <AriaSelect {...mergeProps(restProps, styles.wrapper())}>
            <Button>
                <div {...styles.value()}>
                    {/*startContent*/}
                    <SelectValue />
                    {/*endContent*/}
                </div>
                <ChevronDown {...styles.triggerIndicator()} />
            </Button>
            <Popover>
                <ListBox items={items ?? []}>{children}</ListBox>
            </Popover>
        </AriaSelect>
    )
}

// //==============================================================================
// // SelectView
// //==============================================================================
// namespace SelectView {
//     export interface Props<T extends object> {
//         props: Select.Props<T>
//     }
// }
//
// export const SelectView = <T,>({ props }: SelectView.Props<T>) => {
//     const { styles } = use(SelectStyleContext)
//     const state = use(SelectStateContext)
//
//     return <></>
//
//     return (
//         <MultiProvider
//             values={[
//                 [DialogTriggerState, state],
//                 [SelectStyleProvider, cachedStyles],
//             ]}
//         >
//             <div
//                 {...mergeProps(
//                     filterDOMProps(props),
//                     focusProps,
//                     styles.container() /*renderProps*/,
//                 )}
//                 ref={props.selectRef}
//             >
//                 <Button
//                     {...triggerProps}
//                     ref={triggerRef}
//                     size={values.size}
//                     variant={values.variant}
//                     // stylexProps={styles.trigger} // .propless() ??
//                 >
//                     {label}
//                 </Button>
//             </div>
//             <Popover
//                 ref={popoverRef}
//                 triggerRef={triggerRef}
//                 portalContainer={triggerRef.current?.ownerDocument.body}
//                 placement={"bottom left"}
//             >
//                 <ListStateContext value={state}>
//                     <ListBoxView
//                         {...menuProps}
//                         state={state}
//                         listBoxRef={listBoxRef}
//                         listBoxProps={{ children: null }}
//                     />
//                 </ListStateContext>
//             </Popover>
//             <HiddenSelect
//                 state={state}
//                 triggerRef={triggerRef}
//                 isDisabled={props.isDisabled ?? false}
//                 label={props.label}
//                 // name={name}
//                 // autoComplete={autoComplete}
//             />
//         </MultiProvider>
//     )
// }

//==============================================================================
// SelectItem
//==============================================================================
export interface SelectItemProps {
    ref?: RefObject<HTMLDivElement>
    id?: Key
    children?: ReactNode
}

export const SelectItem = (props: SelectItemProps) => {
    return <ListBoxItem {...props} />
}
