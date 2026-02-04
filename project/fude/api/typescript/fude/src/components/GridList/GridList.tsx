import { filterDOMProps } from "@react-aria/utils"
import type { DragPreviewRenderer, Key, LinkDOMProps } from "@react-types/shared"
import { type ReactNode, type RefObject, use, useRef } from "react"
import {
    type AriaGridListProps,
    type DroppableCollectionResult,
    FocusScope,
    ListKeyboardDelegate,
    mergeProps,
    useCollator,
    useFocusRing,
    useGridList,
    useGridListItem,
    useHover,
} from "react-aria"
import {
    type DraggableCollectionState,
    type DroppableCollectionState,
    type ListState,
    type Node,
    useListState,
} from "react-stately"

import { createNewControlledContext, DEFAULT_SLOT } from "../../hooks/context"
import {
    DragAndDropContext,
    type DragAndDropHooks,
    DropIndicator,
    DropIndicatorContext,
    type DropIndicatorProps,
    useDragAndDropContext,
    useRenderDropIndicator,
} from "../../hooks/useDragAndDrop"
import { useObjectRef } from "../../hooks/useObjectRef"
import { useRenderProps } from "../../hooks/useRenderProps"
import { type VariantProps, useStyles } from "../../theme/props"
import type { Collection } from "../../utils/collection/Collection"
import { createCollectionComponent, type ItemRenderProps } from "../../utils/collection/hooks"
import { MultiProvider } from "../../utils/context"

import type { RenderProps, StyleProps } from "../../utils/props"
import { GridListStyleProvider, gridListStyles } from "./GridList.styles"
import { CollectionBuilder, CollectionItems } from "../../utils/collection/components"
import { CollectionRenderer } from "../../utils/collection/context"
import { ListStateContext } from "../ListBox/ListBox"
import { ButtonContext } from "react-aria-components"

//==============================================================================
// GridList
//==============================================================================
const GridListContext = createNewControlledContext<GridList.Props<any>, HTMLDivElement>()

export namespace GridList {
    export interface Props<T>
        extends AriaGridListProps<T>,
            StyleProps,
            VariantProps<typeof gridListStyles> {
        ref?: RefObject<HTMLDivElement>

        "aria-label": string
        /**
         * The drag and drop hooks returned by `useDragAndDrop`
         */
        dragAndDropHooks?: DragAndDropHooks
        /**
         * A component to render when the list is empty
         */
        renderEmptyState?: () => ReactNode
    }
}

export const GridList = <T extends object>(rawProps: GridList.Props<T>) => {
    const [props, ref] = GridListContext.useContext(rawProps)

    const styles = useStyles(gridListStyles, props)

    return (
        <CollectionBuilder items={<CollectionItems {...props} />}>
            {(collection) => (
                <GridListStyleProvider value={styles}>
                    <GridListView props={props} collection={collection} gridListRef={ref} />
                </GridListStyleProvider>
            )}
        </CollectionBuilder>
    )
}

//==============================================================================
// GridListView
//==============================================================================
namespace GridListView {
    export interface Props<T> {
        props: GridList.Props<T>
        collection: Collection<T>
        gridListRef: RefObject<HTMLDivElement>
    }
}

const GridListView = <T extends object>(rawProps: GridListView.Props<T>) => {
    const { props, collection, gridListRef } = rawProps

    const state = useListState({ ...props, collection, children: [] })
    const collator = useCollator({ usage: "search", sensitivity: "base" })

    const { gridProps } = useGridList(
        {
            ...props,
        },
        state,
        gridListRef,
    )

    const { CollectionRoot } = use(CollectionRenderer)

    const dragAndDropHooks = props.dragAndDropHooks

    let dragState: DraggableCollectionState | undefined = undefined
    let dropState: DroppableCollectionState | undefined = undefined
    let droppableCollection: DroppableCollectionResult | undefined = undefined
    let isRootDropTarget = false
    let dragPreview: React.JSX.Element | null = null
    const preview = useRef<DragPreviewRenderer>(null)

    if (dragAndDropHooks?.drag) {
        dragState = dragAndDropHooks.drag.useDraggableCollectionState({
            collection,
            selectionManager: state.selectionManager,
            preview,
        })
        dragAndDropHooks.drag.useDraggableCollection({}, dragState, gridListRef)

        dragPreview = dragAndDropHooks.drag.renderPreview ? (
            <dragAndDropHooks.drag.Preview ref={preview}>
                {dragAndDropHooks.drag.renderPreview}
            </dragAndDropHooks.drag.Preview>
        ) : null
    }

    if (dragAndDropHooks?.drop) {
        dropState = dragAndDropHooks.drop.useDroppableCollectionState({
            collection,
            selectionManager: state.selectionManager,
        })

        const keyboardDelegate = new ListKeyboardDelegate({
            collection,
            disabledKeys: state.selectionManager.disabledKeys,
            disabledBehavior: state.selectionManager.disabledBehavior,
            ref: gridListRef,
        })

        const dropTargetDelegate =
            dragAndDropHooks.drop.dropTargetDelegate ||
            new dragAndDropHooks.drop.ListDropTargetDelegate(collection, gridListRef)

        droppableCollection = dragAndDropHooks.drop.useDroppableCollection(
            {
                keyboardDelegate,
                dropTargetDelegate,
            },
            dropState,
            gridListRef,
        )

        isRootDropTarget = dropState.isDropTarget({ type: "root" })
    }

    const { styles } = GridListStyleProvider.useContext()

    return (
        <FocusScope>
            <div
                {...mergeProps(
                    filterDOMProps(props),
                    gridProps,
                    droppableCollection?.collectionProps,
                    styles.container(),
                )}
                ref={gridListRef}
            >
                <MultiProvider
                    values={[
                        [ListStateContext, state],
                        [DragAndDropContext, { dragAndDropHooks, dragState, dropState }],
                        [DropIndicatorContext, { render: GridItemDropIndicator }],
                    ]}
                >
                    <CollectionRoot
                        collection={collection}
                        renderDropIndicator={useRenderDropIndicator(dragAndDropHooks, dropState)}
                    />
                </MultiProvider>
                <EmptyState state={state} render={props.renderEmptyState} />
                {dragPreview}
            </div>
        </FocusScope>
    )
}

///=============================================================================
/// GridListItem
///=============================================================================
export namespace GridListItem {
    export interface Props<T> extends RenderProps<ItemRenderProps>, LinkDOMProps {
        ref?: RefObject<HTMLDivElement>
        children?: ReactNode

        textValue?: string
        id: Key
    }
}

export const GridListItem = createCollectionComponent(
    "item",
    <T extends object>(props: GridListItem.Props<T>, node: Node<GridListItem.Props<T>>) => {
        const ref = useObjectRef<HTMLDivElement>(props.ref)

        const state = ListStateContext.useGuaranteedContext()

        const { rowProps, gridCellProps, descriptionProps, ...states } = useGridListItem(
            {
                node,
            },
            state,
            ref,
        )

        const { hoverProps, isHovered } = useHover({})
        const { isFocusVisible, focusProps } = useFocusRing()

        const renderProps = useRenderProps(props, {
            ...states,
            isHovered,
            isFocusVisible,
            selectionMode: "single",
            selectionBehavior: "replace",
        })

        /**
         *
         */
        const { dragAndDropHooks, dragState, dropState } = useDragAndDropContext()

        const showDropIndicator = dragAndDropHooks?.drop?.useDropIndicator
        const isLastKey = state.collection.getKeyAfter(node.key) == null
        const renderDropIndicator =
            dragAndDropHooks?.drop?.renderDropIndicator ||
            ((target) => <DropIndicator target={target} />)

        const draggableItem =
            dragState && dragAndDropHooks
                ? dragAndDropHooks?.drag?.useDraggableItem({ key: node.key }, dragState)
                : undefined

        const { styles } = GridListStyleProvider.useContext()

        return (
            <>
                <div
                    {...mergeProps(
                        filterDOMProps(props as any), // TODO: Fix
                        rowProps,
                        focusProps,
                        hoverProps,
                        styles.item(),
                        draggableItem?.dragProps,
                    )}
                    {...renderProps}
                    ref={ref}
                >
                    <MultiProvider
                        values={[
                            [
                                ButtonContext,
                                {
                                    slots: {
                                        [DEFAULT_SLOT]: {},
                                        drag: {
                                            ...draggableItem?.dragButtonProps,
                                            style: {
                                                pointerEvents: "none",
                                            },
                                        },
                                    },
                                },
                            ],
                        ]}
                    >
                        {renderProps.children}
                    </MultiProvider>
                </div>
            </>
        )
    },
)

////////////////////////////////////////////////////////////////////////////////
/// Drag & Drop
////////////////////////////////////////////////////////////////////////////////
const GridItemDropIndicator = (props: DropIndicatorProps) => {
    const ref = useObjectRef(props.ref)

    const { dragAndDropHooks, dropState } = useDragAndDropContext()

    if (!dragAndDropHooks || !dragAndDropHooks.drop || !dropState) {
        return null
    }

    const { dropIndicatorProps, isHidden, isDropTarget } = dragAndDropHooks.drop.useDropIndicator(
        props,
        dropState,
        ref,
    )

    if (isHidden) {
        return null
    }

    return (
        <GridItemDropIndicatorView
            {...props}
            ref={ref as RefObject<HTMLDivElement>}
            dropIndicatorProps={dropIndicatorProps}
            isDropTarget={isDropTarget}
        />
    )
}

interface GridItemDropIndicatorProps extends DropIndicatorProps {
    ref: RefObject<HTMLDivElement>
    dropIndicatorProps: React.HTMLAttributes<HTMLElement>
    isDropTarget: boolean
}

const GridItemDropIndicatorView = (props: GridItemDropIndicatorProps) => {
    const { styles } = GridListStyleProvider.useContext()
    const renderProps = useRenderProps(props, { isDropTarget: props.isDropTarget })

    return (
        <div
            {...mergeProps(props.dropIndicatorProps, styles.dropIndicator(), renderProps)}
            ref={props.ref}
            data-drop-target={props.isDropTarget || undefined}
        />
    )
}

///=============================================================================
/// EmptyState
///=============================================================================
namespace EmptyState {
    export interface Props {
        state: ListState<unknown>
        render?: (() => ReactNode) | undefined
    }
}

const EmptyState = (props: EmptyState.Props) => {
    if (props.state.collection.size !== 0 || !props.render) {
        return null
    }

    return (
        <div role="row" style={{ display: "contents" }}>
            <div role="gridcell" style={{ display: "contents" }}>
                {props.render()}
            </div>
        </div>
    )
}
