import { mergeProps, useObjectRef } from "@react-aria/utils"
import { createContext, type ReactNode, type RefObject, use } from "react"
import type { Key } from "react-aria"
import {
    type MenuItemProps,
    type MenuProps,
    type MenuTriggerProps,
    Menu as RacMenu,
    MenuItem as RacMenuItem,
    MenuTrigger as RacMenuTrigger,
} from "react-aria-components"
import {
    type Node,
    type RootMenuTriggerState,
    type SubmenuTriggerProps,
    type TreeState,
    useMenuTriggerState,
    useSubmenuTriggerState,
    useTreeState,
} from "react-stately"
import {
    createControlledContext,
    createGenericContext,
    createNewControlledContext,
    createNewGenericContext,
} from "../../hooks/context"
import { useStyles, type VariantProps } from "../../theme/props"
import {
    type Collect,
    createCollectionComponent,
    type ItemRenderProps,
} from "../../utils/collection/hooks"
import { MultiProvider } from "../../utils/context"
import type { NamedStyleSlots } from "../../utils/props"
import { MenuStyleContext, menuStyles } from "./Menu.styles"

//==============================================================================
// Menu Support
//==============================================================================
const MenuInlineContext = createContext(false)

//==============================================================================
// MenuTrigger
//==============================================================================
export namespace MenuTrigger {
    export interface Props extends MenuTriggerProps {
        inline?: boolean
    }
}

export const MenuTrigger = (props: MenuTrigger.Props): ReactNode => {
    return (
        <MenuInlineContext value={!!props.inline}>
            <RacMenuTrigger {...props} />
        </MenuInlineContext>
    )
}

// //==============================================================================
// // SubmenuTrigger
// //==============================================================================
// const SubmenuTriggerContext = createNewGenericContext<{
//     parentMenuRef: RefObject<HTMLElement | null>
// }>()
//
// export namespace SubmenuTrigger {
//     export interface Props extends SubmenuTriggerProps {
//         ref?: RefObject<HTMLDivElement>
//         children: ReactElement[]
//     }
// }
//
// export const SubmenuTrigger = createCollectionComponent(
//     "submenutrigger",
//     (props: SubmenuTrigger.Props, node: Node<unknown>) => {
//         const { CollectionNode } = use(CollectionRenderer)
//         const state = MenuState.useContext()
//
//         const rootTriggerState = RootTriggerState.useContext()
//         const nodeTriggerState = useSubmenuTriggerState({ triggerKey: node.key }, rootTriggerState)
//
//         const submenuRef = useObjectRef<HTMLDivElement>()
//         const itemRef = useObjectRef(props.ref)
//         // const popoverContext = useSlottedContext(PopoverContext)!
//         const { parentMenuRef } = SubmenuTriggerContext.useContext()
//         const { submenuTriggerProps, submenuProps, popoverProps } = useSubmenuTrigger(
//             {
// //                 parentMenuRef,
//                 submenuRef,
//                 delay: 200,
//             },
//             nodeTriggerState,
//             itemRef,
//         )
//
//         return (
//             <MultiProvider
//                 values={[
//                     [
//                         MenuItemContext,
//                         { ...submenuTriggerProps, onAction: undefined, ref: itemRef },
//                     ],
//                     [MenuContext, submenuProps as never as Menu.Props<any>],
//                     [
//                         PopoverContext,
//                         {
//                             ref: submenuRef,
//                             triggerRef: itemRef,
//                             placement: "end top",
//                             ...popoverProps,
//                         },
//                     ],
//                 ]}
//             >
//                 <CollectionNode collection={state.collection} parent={node} />
//
//                 {nodeTriggerState.isOpen && props.children[1]}
//             </MultiProvider>
//         )
//     },
//     (props) => props.children[0],
// )

//==============================================================================
// Menu
//==============================================================================
export namespace Menu {
    export interface Props<T> extends MenuProps<T>, VariantProps<typeof menuStyles> {
        ref?: RefObject<HTMLDivElement>
    }
}

export const Menu = <T extends object>(props: Menu.Props<T>) => {
    const styles = useStyles(menuStyles, props)

    return (
        <MenuStyleContext value={styles}>
            <RacMenu {...mergeProps(props, styles.styles.menuWrapper())} />
        </MenuStyleContext>
    )
}

// //==============================================================================
// // MenuSection
// //==============================================================================
// export namespace MenuSection {
//     export interface Props<T> extends NamedStyleSlots<"section"> {
//         item: Node<T>
//         children?: ReactNode
//     }
// }
//
// const MenuSection = createCollectionComponent(
//     "section",
//     <T extends object>(props: MenuSection.Props<T>, node: Node<MenuSection.Props<T>>) => {
//         const state = MenuState.useContext()
//
//         const headingRef = useRef(undefined)
//         const { headingProps, groupProps } = useMenuSection({})
//
//         const { CollectionNode } = useContext(CollectionRenderer)
//
//         /*
//         const { styles } = useMenuStyles()
//         const children = useCachedChildren({
//             items: state.collection.getChildren?.(props.item.key) ?? [],
//             children: (item) => {
//                 switch (item.type) {
//                     case "item":
//                         return <MenuItemView item={item} />
//                     case "header":
//                         // {props.item.rendered}
//
// //                         return (
//                             <Header
//                                 {...mergeProps(
//                                     headingProps,
//                                     props.item.props,
//                                     styles.sectionHeader(),
//                                 )}
//                                 ref={mergeRefs(headingRef, props.item.props.ref)}
//                             >
//                                 {item.rendered}
//                             </Header>
//                         )
//                     default:
//                         throw new Error(`Unknown node type in MenuSection: ${item.type}`)
//                 }
//             },
//         })
//         */
//
//         return (
//             <section {...groupProps}>
//                 <CollectionNode collection={state.collection} parent={node} />
//             </section>
//         )
//     },
// )

//==============================================================================
// MenuItem
//==============================================================================
export namespace MenuItem {
    export interface Props
        extends Omit<MenuItemProps, "id" | "children">,
            NamedStyleSlots<"item" | "label" | "description" | "shortcut"> {
        ref?: RefObject<HTMLDivElement>
        label?: string | ReactNode
        key?: Key | undefined
        id?: string
        onClick?(): void
        children: ReactNode
        /*
                    label?: string
    description?: string
    shortcut?: string

    before?: ReactNode
    after?: ReactNode
    */
    }
}

export const MenuItem = (props: MenuItem.Props) => {
    const ref = useObjectRef(props.ref)

    const { styles } = use(MenuStyleContext)
    let { label, ...restProps } = props

    return (
        <RacMenuItem {...mergeProps(props, styles.itemWrapper())} ref={ref}>
            <MultiProvider
                values={
                    [
                        // [
                        //     TextContext,
                        //     {
                        //         slots: {
                        //             label: {
                        //                 ...labelProps,
                        //                 ...styles.itemLabel(),
                        //             },
                        //             description: {
                        //                 ...descriptionProps,
                        //                 ...styles.itemDescription(),
                        //             },
                        //         },
                        //     },
                        // ],
                        // [
                        //     KeyboardContext,
                        //     {
                        //         ...keyboardShortcutProps,
                        //         ...styles.itemShortcut(),
                        //     },
                        // ],
                    ]
                }
            >
                <div>{label}</div>
                {/*props.before*/}
                {/*props.description ? (
                        <div {...styles.itemContent()}>
                            <Text slot="label">{label}</Text>
                            <Text slot="description">{props.description}</Text>
                        </div>
                    ) : (
                        <Text slot="label">{label}</Text>
                    )*/}
                {/*props.shortcut && <Keyboard>{props.shortcut}</Keyboard>*/}
                {/*props.after*/}
                {props.children}
            </MultiProvider>
        </RacMenuItem>
    )
}
