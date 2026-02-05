import { type BaseCollection, createLeafComponent } from "@react-aria/collections"
import { filterDOMProps, useId } from "@react-aria/utils"
import type { CollectionBase, GlobalDOMAttributes } from "@react-types/shared"
import { LayoutGroup, motion } from "framer-motion"
import { type ForwardedRef, type ReactNode, type RefObject, use, useRef } from "react"
import {
    type AriaTabListProps,
    type AriaTabPanelProps,
    type Key,
    mergeProps,
    useFocusRing,
    useHover,
    useTab,
    useTabList,
    useTabPanel,
} from "react-aria"
import { Collection, CollectionBuilder, CollectionRendererContext } from "react-aria-components"
import { type Node, type TabListState, useTabListState } from "react-stately"

import { createGenericContext } from "../../hooks/context.js"
import { type ContextMenuProps, useContextMenu } from "../../hooks/useContextMenu.js"
import { useObjectRef } from "../../hooks/useObjectRef.js"
import { useRenderProps, useStyleProps } from "../../hooks/useRenderProps.js"
import { useStyles, type VariantProps } from "../../theme/props.js"
import { MultiProvider } from "../../utils/context.js"
import type { RenderProps, StyleProps } from "../../utils/props.js"
import { TabsStyleProvider, tabsStyles, useTabsStyles } from "./Tabs.stylex.js"

//==============================================================================
// Tabs
//==============================================================================
/**
 * TODO: Docs
 * @alpha
 */
export const [useTabsContext, TabsContext] = createGenericContext<{
    state: TabListState<object>
}>()

export namespace Tabs {
    export interface Props<T extends object>
        extends CollectionBase<AriaTabListProps<T>>,
            GlobalDOMAttributes<HTMLDivElement>,
            StyleProps,
            VariantProps<typeof tabsStyles> {
        ref?: RefObject<HTMLDivElement>
        addons?: ReactNode
        defaultSelectedKey?: Key | undefined
    }
}

export const Tabs = <T extends object>(props: Tabs.Props<T>) => {
    return (
        <CollectionBuilder content={<Collection {...props} />}>
            {(collection: BaseCollection<T>) => (
                <TabsView<T> tabsProps={props} collection={collection} />
            )}
        </CollectionBuilder>
    )
}

//==============================================================================
// TabsView
//==============================================================================
export namespace TabsView {
    export interface Props<T extends object> {
        tabsProps: Tabs.Props<T>
        collection: BaseCollection<T>
    }
}

const TabsView = <T extends object>(props: TabsView.Props<T>) => {
    const state = useTabListState({
        ...props,
        defaultSelectedKey: props.tabsProps.defaultSelectedKey as never, // Required because we override the type for exactOptionalProperties=true
        collection: props.collection,
        children: [],
    })

    const cachedStyles = useStyles(tabsStyles, props.tabsProps)
    const styles = cachedStyles.styles

    const styleProps = useStyleProps(props.tabsProps, {})

    return (
        <MultiProvider
            values={[
                [TabsContext, { state }],
                [TabsStyleProvider, cachedStyles],
            ]}
        >
            <div
                {...mergeProps(filterDOMProps(props.tabsProps), styles.wrapper(), styleProps)}
                ref={props.tabsProps.ref}
            >
                <TabList addons={props.tabsProps.addons} />
                <TabPanel />
            </div>
        </MultiProvider>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// Tab
////////////////////////////////////////////////////////////////////////////////
export interface TabRenderProps {
    /** Whether the tab is disabled */
    isDisabled: boolean
    /** Whether the tab is currently focused by either mouse or keyboard */
    isFocused: boolean
    /** Whether the tab is currently focused by keyboard */
    isFocusVisible: boolean
    /** Whether the tab is currently being hovered. */
    isHovered: boolean
    /** Whether the tab is currently in a pressed state. */
    isPressed: boolean
    /** Whether the tab is currently selected. */
    isSelected: boolean
}

export interface TabProps extends RenderProps<TabRenderProps>, ContextMenuProps {
    ref?: RefObject<HTMLElement>

    id: Key
    /**
     * An optional icon that will be displayed to the left of the tab content
     */
    icon?: ReactNode
    /**
     *
     */
    title: ReactNode

    children?: ReactNode
}

export const Tab = createLeafComponent(
    "item",
    (_props: TabProps, nodeRef: ForwardedRef<HTMLDivElement>, node: Node<TabProps>) => {
        const { state } = useTabsContext()
        const { styles } = useTabsStyles()

        const props = node.props as TabProps
        const ref = useObjectRef<HTMLDivElement>(nodeRef)

        const { tabProps, isDisabled, isPressed, isSelected } = useTab(
            { key: node.key },
            state,
            ref,
        )

        const { focusProps, isFocused, isFocusVisible } = useFocusRing()
        const { hoverProps, isHovered } = useHover({ isDisabled })

        const { contextMenuProps } = useContextMenu(props)

        const renderProps = useRenderProps(
            { ...props, children: node.rendered },
            {
                isDisabled,
                isFocused,
                isFocusVisible,
                isHovered,
                isPressed,
                isSelected,
            },
        )

        return (
            <>
                <div
                    {...mergeProps(
                        tabProps,
                        focusProps,
                        hoverProps,
                        contextMenuProps,
                        styles.tabItem(),
                    )}
                    ref={ref}
                    data-disabled={isDisabled || undefined}
                    data-focused={isFocused || undefined}
                    data-focus-visible={isFocusVisible || undefined}
                    data-hovered={isHovered || undefined}
                    data-pressed={isPressed || undefined}
                    data-selected={isSelected || undefined}
                >
                    {isSelected && (
                        <motion.div
                            {...styles.cursor()}
                            layoutId="cursor"
                            transition={{
                                type: "spring",
                                bounce: 0.2,
                                duration: 0.5,
                            }}
                        />
                    )}
                    <div {...mergeProps(styles.tabContent(), renderProps)}>
                        {props.icon}
                        {props.title}
                    </div>
                </div>
            </>
        )
    },
)

////////////////////////////////////////////////////////////////////////////////
/// TabList
////////////////////////////////////////////////////////////////////////////////
interface TabListProps<T extends object> extends Omit<AriaTabListProps<T>, "children">, StyleProps {
    addons?: ReactNode
}

const TabList = <T extends object>(props: TabListProps<T>) => {
    const { CollectionRoot } = use(CollectionRendererContext)

    const { styles } = useTabsStyles()

    const { state } = useTabsContext()

    const tabsRef = useRef<HTMLDivElement>(null)

    const { tabListProps } = useTabList(props, state, tabsRef)

    const layoutId = useId()

    /*
    {[...state.collection].map((it) => (
                            <TabView key={it.key} item={it} state={state} />
                        ))}
                        */
    return (
        <LayoutGroup id={layoutId}>
            <div
                {...mergeProps(filterDOMProps(props), tabListProps, styles.tabList())}
                ref={tabsRef}
                // className={props.className}
                // style={props.style}
            >
                <div {...styles.tabListFlexWrapper()}>
                    <div {...styles.tabListItems()}>
                        <CollectionRoot collection={state.collection} />
                    </div>
                    <div {...styles.tabListAddons()}>{props.addons}</div>
                </div>
            </div>
        </LayoutGroup>
    )
}

////////////////////////////////////////////////////////////////////////////////
/// TabPanel
////////////////////////////////////////////////////////////////////////////////
export interface TabPanelRenderProps {
    /** Whether the TabPanel is actively focused. */
    isFocused: boolean
    /** Whether the TabPanel is actively focused using the keyboard. */
    isFocusVisible: boolean
    /** Internal TabList state. */
    state: TabListState<unknown>
}

export interface TabPanelProps extends AriaTabPanelProps, RenderProps<TabPanelRenderProps> {}

const TabPanel = (props: TabPanelProps) => {
    const { styles } = useTabsStyles()

    const { state } = useTabsContext()

    const ref = useObjectRef<HTMLDivElement>()

    const { tabPanelProps } = useTabPanel(props, state, ref)
    const { focusProps, isFocused, isFocusVisible } = useFocusRing()

    const renderProps = useRenderProps(props, { isFocused, isFocusVisible, state })

    const selectedItem = state.selectedItem
    const content = selectedItem?.props?.children

    if (!content) {
        return null
    }

    return (
        <div
            {...mergeProps(
                filterDOMProps(props as never),
                tabPanelProps,
                focusProps,
                renderProps,
                styles.panel(),
            )}
            ref={ref}
            data-focused={isFocused || undefined}
            data-focus-visible={isFocusVisible || undefined}
        >
            {content}
        </div>
    )
}
