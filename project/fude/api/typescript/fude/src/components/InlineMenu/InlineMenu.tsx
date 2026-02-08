import type { AriaMenuProps } from "react-aria"
import {
    createContext,
    use,
    useCallback,
    useMemo,
    useRef,
    useState,
    type ReactNode,
    type RefObject,
} from "react"

import {
    createCollectionComponent,
    type Collect,
    type CollectionProps,
    type ItemRenderProps,
} from "../../utils/collection/hooks"
import type { RenderProps, StyleProps, StyleSlots } from "../../utils/props"
import type { VariantProps } from "../../theme/props"
import { CollectionBuilder, CollectionItems } from "../../utils/collection/components"
import type { Collection } from "../../utils/collection/document"
import {
    useTreeState,
    type MenuTriggerProps,
    type Node,
    type RootMenuTriggerState,
    type SubmenuTriggerProps,
    type SubmenuTriggerState,
} from "react-stately"
import { useObjectRef } from "../../hooks/useObjectRef"

import { create } from "@stylexjs/stylex"

import { padding } from "../../theme/atomics/padding"
import { type CachedStyles, makeStyles, useStyles } from "../../theme/props"
import { borderRadius } from "@sribich/fude-theme/vars/borderRadius.stylex"
import { borderWidth } from "@sribich/fude-theme/vars/borderWidth.stylex"
import { colors } from "@sribich/fude-theme/vars/colors.stylex"
import { fontSize } from "@sribich/fude-theme/vars/fontSize.stylex"
import { spacing } from "@sribich/fude-theme/vars/spacing.stylex"
import { CollectionRenderer } from "../../utils/collection/context"
import { useFocusRing, useMenu, useMenuItem, useSubmenuTrigger } from "react-aria"
import { mergeProps } from "../../utils/mergeProps"
import { filterDOMProps } from "@react-aria/utils"
import { useRenderProps, useStyleProps } from "../../hooks/useRenderProps"
import { MultiProvider } from "../MultiProvider"
import { DividerContext } from "../Divider/Divider"
import type { FocusStrategy, Key, LinkDOMProps } from "@react-types/shared"
import { createControlledContext, createGenericContext } from "../../hooks/context"
import { Text, TextContext } from "../Text/primitive/Text"
import { Keyboard, KeyboardContext } from "../Keyboard/primitive/Keyboard"

// export const [useMenuState, MenuState] = createGenericContext<TreeState<unknown>>()

/**
 * Used to keep track of the root triggger state in place of the root
 * trigger component, which does not exist in an inline menu.
 *
 * The context will need to be explicitly read and passed down in every
 * menu component as it needs to know if state exists in order to set the
 * initial state.
 */
const RootTriggerState = createContext<RootMenuTriggerState | null>(null)

const useMenuTriggerState = (): RootMenuTriggerState => {
    const [expandedKeys, setExpandedKeys] = useState<Key[]>([])

    const closeAll = () => {
        setExpandedKeys([])
    }

    const openSubmenu = (triggerKey: Key, level: number) => {
        setExpandedKeys((oldStack) => {
            return [...oldStack, triggerKey]

            /*
            if (level > oldStack.length) {
                return oldStack
            }
            return [...oldStack.slice(0, level), triggerKey]
            */
        })
    }

    const closeSubmenu = (triggerKey: Key, level: number) => {
        setExpandedKeys((oldStack) => {
            return oldStack.filter((it) => it !== triggerKey)
            /*
            let key = oldStack[level]
            if (key === triggerKey) {
                return oldStack.slice(0, level)
            } else {
                return oldStack
            }
            */
        })
    }

    return {
        close() {
            closeAll()
        },
        expandedKeys,
        expandedKeysStack: expandedKeys,
        openSubmenu,
        closeSubmenu,
    }
}

const useSubmenuTriggerState = (
    props: SubmenuTriggerProps,
    state: RootMenuTriggerState,
): SubmenuTriggerState => {
    let { triggerKey } = props
    let { expandedKeysStack, openSubmenu, closeSubmenu, close: closeAll } = state
    let [submenuLevel] = useState(expandedKeysStack?.length)
    let isOpen = useMemo(
        () => expandedKeysStack.some((it) => it === triggerKey),
        [expandedKeysStack, triggerKey, submenuLevel],
    )
    let [focusStrategy, setFocusStrategy] = useState<FocusStrategy>(null)

    let open = useCallback(
        (focusStrategy: FocusStrategy = null) => {
            setFocusStrategy(focusStrategy)
            openSubmenu(triggerKey, submenuLevel)
        },
        [openSubmenu, submenuLevel, triggerKey],
    )

    let close = useCallback(() => {
        setFocusStrategy(null)
        closeSubmenu(triggerKey, submenuLevel)
    }, [closeSubmenu, submenuLevel, triggerKey])

    let toggle = useCallback(
        (focusStrategy: FocusStrategy = null) => {
            setFocusStrategy(focusStrategy)
            if (isOpen) {
                close()
            } else {
                open(focusStrategy)
            }
        },
        [close, open, isOpen],
    )

    return useMemo(
        () => ({
            focusStrategy,
            isOpen,
            open,
            close,
            closeAll,
            submenuLevel,
            // TODO: Placeholders that aren't used but give us parity with OverlayTriggerState so we can use this in Popover. Refactor if we update Popover via
            // https://github.com/adobe/react-spectrum/pull/4976#discussion_r1336472863
            setOpen: () => {},
            toggle,
        }),
        [isOpen, open, close, closeAll, focusStrategy, toggle, submenuLevel],
    )
}

///==============================================================================
/// InlineMenu
///==============================================================================
export namespace InlineMenu {
    export interface Props<T>
        extends Collect<AriaMenuProps<T>>,
            StyleProps,
            VariantProps<typeof inlineMenuStyles> {
        ref?: RefObject<HTMLDivElement>
    }
}

export const InlineMenu = <T extends object>(props: InlineMenu.Props<T>): ReactNode => {
    // This will not exist for the root menu, but will for nested submenus.
    const context = use(RootTriggerState)
    const state = context ?? useMenuTriggerState()

    return (
        <CollectionBuilder items={<CollectionItems {...props} />}>
            {(collection) =>
                collection.isPopulated() && (
                    <RootTriggerState value={state}>
                        <InlineMenuView collection={collection} menuProps={props} />
                    </RootTriggerState>
                )
            }
        </CollectionBuilder>
    )
}

///==============================================================================
/// InlineMenuView
///==============================================================================
namespace InlineMenuView {
    export interface Props<T> {
        collection: Collection<T>
        menuProps: InlineMenu.Props<T>
    }
}

const InlineMenuView = <T extends object>(props: InlineMenuView.Props<T>) => {
    const state = useTreeState({
        collection: props.collection,
        children: [],
    })

    const menuRef = useObjectRef(props.menuProps.ref)
    const { menuProps } = useMenu(
        {
            ...props.menuProps,
        },
        state,
        menuRef,
    )

    const styles = useStyles(inlineMenuStyles, props.menuProps)

    const { CollectionRoot } = use(CollectionRenderer)

    const styleProps = useStyleProps(props.menuProps, {})

    return (
        <div
            {...mergeProps(
                filterDOMProps(props.menuProps),
                menuProps,
                styles.styles.menuWrapper(),
                styleProps,
            )}
            ref={menuRef}
        >
            <MultiProvider
                values={[
                    [MenuState, state],
                    [DividerContext, { elementType: "div" }],
                    [InlineMenuStyleProvider, styles],
                    [SubmenuTriggerContext, { parentMenuRef: menuRef }],
                    [MenuItemContext, null],
                ]}
            >
                <CollectionRoot collection={props.collection} />
            </MultiProvider>
        </div>
    )
}

///==============================================================================
/// InlineMenuItem
///==============================================================================
/*
export interface MenuSectionProps<T> extends StyleSlots<"section"> {
    item: Node<T>
}

const MenuSection = <T,>(props: MenuSectionProps<T>) => {
    const { styles } = useMenuStyles()

    const state = useMenuState()

    const headingRef = useRef()
    const { headingProps, groupProps } = useMenuSection({})

    const children = useCachedChildren({
        items: state.collection.getChildren?.(props.item.key) ?? [],
        children: (item) => {
            switch (item.type) {
                case "item":
                    return <MenuItemView item={item} />
                case "header":
                    // {props.item.rendered}

                    return (
                        <Header
                            {...mergeProps(headingProps, props.item.props, styles.sectionHeader())}
                            ref={mergeRefs(headingRef, props.item.props.ref)}
                        >
                            {item.rendered}
                        </Header>
                    )
                default:
                    throw new Error(`Unknown node type in MenuSection: ${item.type}`)
            }
        },
    })

    return <section {...groupProps}>{children}</section>
}
*/

///==============================================================================
/// InlineMenuItem
///==============================================================================
export namespace InlineMenuItem {
    export interface Props
        extends RenderProps<ItemRenderProps>,
            LinkDOMProps,
            StyleSlots<"item" | "label" | "description" | "shortcut"> {
        ref?: RefObject<HTMLDivElement>

        key?: Key
        id?: Key
        onClick?(): void
    }
}
/*
export interface MenuItemProps<T> extends Omit<MenuItemProps<T>, "className" | "style">, StyleProps {
    label?: string
    description?: string
    shortcut?: string

    before?: ReactNode
    after?: ReactNode
}

*/

const [useMenuItemContext, MenuItemContext] = createControlledContext<
    InlineMenuItem.Props,
    HTMLDivElement
>()

// createContext<ContextValue<MenuItemProps, HTMLDivElement>>(null)

export const InlineMenuItem = createCollectionComponent(
    "item",
    (_props: InlineMenuItem.Props, node: Node<InlineMenuItem.Props>) => {
        const [props, fref] = useMenuItemContext(_props)
        const ref = useObjectRef(fref)

        const { styles } = useInlineMenuStyles()

        const state = MenuState.useContext()

        const { menuItemProps, labelProps, descriptionProps, keyboardShortcutProps, ...states } =
            useMenuItem({ key: node.key, id: props.id, ...props }, state, ref)

        const { isFocusVisible, focusProps } = useFocusRing()

        const renderProps = useRenderProps(
            {
                ...props,
                children: node.rendered,
            },
            {},
        )

        let { label, ...restProps } = props

        if (restProps.children) {
            label = renderProps.children
        }

        const ElementType = restProps.href ? "a" : "div"

        const isToggleMenu = "$$open" in props
        const isOpen = props.$$open

        return (
            <ElementType
                {...mergeProps(menuItemProps, focusProps, renderProps, styles.itemWrapper())}
                ref={ref as any}
                data-disabled={states.isDisabled || undefined}
                data-focused={states.isFocused || undefined}
                data-focus-visible={isFocusVisible || undefined}
                data-hovered={states.isFocused || undefined}
                data-pressed={states.isPressed || undefined}
                data-selected={states.isSelected || undefined}
            >
                <MultiProvider
                    values={[
                        [
                            TextContext,
                            {
                                slots: {
                                    label: {
                                        ...labelProps,
                                        ...styles.itemLabel(),
                                    },
                                    description: {
                                        ...descriptionProps,
                                        ...styles.itemDescription(),
                                    },
                                },
                            },
                        ],
                        [
                            KeyboardContext,
                            {
                                ...keyboardShortcutProps,
                                ...styles.itemShortcut(),
                            },
                        ],
                    ]}
                >
                    {props.before}
                    {props.description ? (
                        <div {...styles.itemContent()}>
                            <Text slot="label">{label}</Text>
                            <Text slot="description">{props.description}</Text>
                        </div>
                    ) : (
                        <Text slot="label">{label}</Text>
                    )}
                    {props.shortcut && <Keyboard>{props.shortcut}</Keyboard>}
                    {props.after}
                    {isToggleMenu && <Indicator open={isOpen} />}
                </MultiProvider>
            </ElementType>
        )
    },
)

const Indicator = (props) => {
    if (props.open) {
        return <span>^f</span>
    }

    return <span>{"vf"}</span>
}

///==============================================================================
/// InlineSubmenuTrigger
///==============================================================================
const SubmenuTriggerContext = createContext<{
    parentMenuRef: RefObject<HTMLElement | null>
} | null>(null)

export namespace InlineSubmenuTrigger {
    export interface Props extends SubmenuTriggerProps {
        ref?: RefObject<HTMLDivElement>
        children: ReactNode
    }
}

export const InlineSubmenuTrigger = createCollectionComponent(
    "trigger",
    (props: InlineSubmenuTrigger.Props, node: Node<unknown>) => {
        const { CollectionNode } = use(CollectionRenderer)
        const state = MenuState.useContext()

        const rootTriggerState = use(RootTriggerState)
        const nodeTriggerState = useSubmenuTriggerState({ triggerKey: node.key }, rootTriggerState)

        const submenuRef = useRef<HTMLDivElement>(null)
        const itemRef = useObjectRef(props.ref)
        // const popoverContext = useSlottedContext(PopoverContext)!
        const { parentMenuRef } = use(SubmenuTriggerContext)
        const { submenuTriggerProps, submenuProps, popoverProps } = useSubmenuTrigger(
            {
                parentMenuRef,
                submenuRef,
                delay: 200,
            },
            nodeTriggerState,
            itemRef,
        )

        const { onHoverChange, onBlur, ...submenuTriggerProps2 } = submenuTriggerProps

        return (
            <MultiProvider
                values={[
                    [
                        MenuItemContext,
                        {
                            $$open: nodeTriggerState.isOpen,
                            ...submenuTriggerProps2,
                            onAction: undefined,
                            ref: itemRef,
                            onPress: nodeTriggerState.isOpen
                                ? nodeTriggerState.close
                                : submenuTriggerProps2.onPress,
                        },
                    ],
                    //                     [MenuContext, submenuProps],
                ]}
            >
                <CollectionNode collection={state.collection} parent={node} />

                {nodeTriggerState.isOpen && props.children[1]}
            </MultiProvider>
        )
    },
    (props) => props.children[0],
)

export const inlineMenuStyles = makeStyles({
    slots: create({
        menuWrapper: {
            width: "auto",
            overflow: "auto",
            padding: spacing["1"],

            outline: "none",

            color: colors.foreground,
            ":hover": {
                color: colors.backgroundHoverForeground,
            },
            // "bg-white ring-1 ring-black ring-opacity-5 animate-in fade-in zoom-in-95 fill-mode-forwards origin-top-left",
        },
        sectionHeader: {
            // "text-foreground-500 pl-1 text-xs"
            paddingLeft: spacing["1"],
            fontSize: fontSize.xs,
        },
        itemWrapper: {
            position: "relative",
            marginBottom: spacing["0.5"],
            boxSizing: "border-box",
            display: "flex",
            width: "100%",
            cursor: "pointer",
            alignItems: "center",
            justifyContent: "space-between",
            outline: "none",
            borderRadius: borderRadius.sm,
            textDecoration: "none",
            color: colors.foreground,
            ":hover": {
                backgroundColor: colors.backgroundHover,
            },

            // item: " relative  items-center justify-betweentext-gray-900 subpixel-antialiased outline-none focus:bg-violet-500 focus:text-white",
        },
        itemContent: {
            display: "flex",
            width: "100%",
            flexDirection: "column",
            alignItems: "flex-start",
            justifyContent: "center",
        },
        itemLabel: {
            flex: "1 1 0%",
            fontSize: fontSize.sm,
            // label: "flex-1 truncate text-sm font-normal",
        },
        itemDescription: {
            // "text-foreground-500 group-hover:text-current"
            width: "100%",
            fontSize: fontSize.xs,
        },
        itemShortcut: {
            borderColor: colors.borderUi,
            borderWidth: borderWidth.sm,
            borderStyle: "solid",
            borderRadius: borderRadius.sm,
            fontSize: fontSize.xs,
            /// ...stylex.include({ ...padding["x-1"], ...padding["y-0.5"] }),
            // ...padding["x-1"],
            // text-foreground-500   font-sans group-hover:border-current
        },
    }),
    conditions: {},
    variants: {
        size: {
            sm: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-1.5"], ...padding["y-1"] }),
                    // item: "h-7",
                    // "[&_[data-menu-item]]:h-6 [&_[data-menu-item]_button]:w-6 [&_[data-menu-item]_button]:h-6",
                },
            }),
            md: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-2"], ...padding["y-2"] }),
                    // item: "h-9",
                    // "[&_[data-menu-item]]:h-7 [&_[data-menu-item]_button]:w-7 [&_[data-menu-item]_button]:h-7",
                },
            }),
            lg: create({
                itemWrapper: {
                    // ...stylex.include({ ...padding["x-3"], ...padding["y-3"] }),
                    // item: "h-11",
                    // "[&_[data-menu-item]]:h-8 [&_[data-menu-item]_button]:w-8 [&_[data-menu-item]_button]:h-8",
                },
            }),
        },
        variant: {
            solid: create({
                menuWrapper: {
                    // borderRadius: borderRadius.md,
                    // padding: spacing["1"],
                    // boxShadow: boxShadow.lg,
                    backgroundColor: colors.backgroundSecondary,
                },
            }),
            light: create({}),
            // light
            // ghost
        },
    },
    defaultVariants: {
        size: "md",
        variant: "solid",
    },
})

export const [useInlineMenuStyles, InlineMenuStyleProvider] =
    createGenericContext<CachedStyles<typeof inlineMenuStyles>>()
