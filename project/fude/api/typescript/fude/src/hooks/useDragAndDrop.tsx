import type { DraggableCollectionProps, DroppableCollectionProps, Key } from "@react-types/shared"
import { type ReactNode, type RefObject, useCallback, useMemo } from "react"
import {
    type DropIndicatorProps as AriaDropIndicatorProps,
    type DragItem,
    DragPreview,
    type DraggableCollectionOptions,
    type DraggableItemProps,
    type DraggableItemResult,
    type DropIndicatorAria,
    type DropTarget,
    type DropTargetDelegate,
    type DroppableCollectionOptions,
    type DroppableCollectionResult,
    type DroppableItemOptions,
    type DroppableItemResult,
    ListDropTargetDelegate,
    useDraggableCollection,
    useDraggableItem,
    useDropIndicator,
    useDroppableCollection,
    useDroppableItem,
} from "react-aria"
import {
    type DraggableCollectionState,
    type DraggableCollectionStateOptions,
    type DroppableCollectionState,
    type DroppableCollectionStateOptions,
    useDraggableCollectionState,
    useDroppableCollectionState,
} from "react-stately"

import type { RenderProps, StyleProps } from "../utils/props"
import { createGenericContext } from "./context"
import type { ItemDropTarget } from "react-aria"

interface DraggableCollectionStateOpts extends Omit<DraggableCollectionStateOptions, "getItems"> {}

interface DragHooks {
    useDraggableCollectionState: (props: DraggableCollectionStateOpts) => DraggableCollectionState
    useDraggableCollection: (
        props: DraggableCollectionOptions,
        state: DraggableCollectionState,
        ref: RefObject<HTMLElement>,
    ) => void
    useDraggableItem: (
        props: DraggableItemProps,
        state: DraggableCollectionState,
    ) => DraggableItemResult

    renderPreview: ((items: DragItem[]) => React.JSX.Element) | undefined
    Preview: typeof DragPreview
}

interface DropHooks {
    useDroppableCollectionState: (
        props: DroppableCollectionStateOptions,
    ) => DroppableCollectionState
    useDroppableCollection: (
        props: DroppableCollectionOptions,
        state: DroppableCollectionState,
        ref: RefObject<HTMLElement>,
    ) => DroppableCollectionResult
    useDroppableItem: (
        options: DroppableItemOptions,
        state: DroppableCollectionState,
        ref: RefObject<HTMLElement>,
    ) => DroppableItemResult
    useDropIndicator: (
        props: AriaDropIndicatorProps,
        state: DroppableCollectionState,
        ref: RefObject<HTMLElement>,
    ) => DropIndicatorAria
    renderDropIndicator: ((target: DropTarget) => React.JSX.Element) | undefined
    dropTargetDelegate: DropTargetDelegate | undefined
    ListDropTargetDelegate: typeof ListDropTargetDelegate
}

export interface DragAndDropHooks {
    drag?: DragHooks
    drop?: DropHooks
}

export interface DragAndDrop {
    dragAndDropHooks: DragAndDropHooks
}

export interface DragAndDropOptions
    extends Omit<DraggableCollectionProps, "preview" | "getItems">,
        DroppableCollectionProps {
    /**
     * A function that returns the items being dragged. If not specified, we assume that the collection is not draggable.
     * @default () => []
     */
    getItems?: (keys: Set<Key>) => DragItem[]
    /**
     * A function that renders a drag preview, which is shown under the user's cursor while dragging.
     * By default, a copy of the dragged element is rendered.
     */
    renderDragPreview?: (items: DragItem[]) => React.JSX.Element
    /**
     * A function that renders a drop indicator element between two items in a collection.
     * This should render a `<DropIndicator>` element. If this function is not provided, a
     * default DropIndicator is provided.
     */
    renderDropIndicator?: (target: DropTarget) => React.JSX.Element
    /** A custom delegate object that provides drop targets for pointer coordinates within the collection. */
    dropTargetDelegate?: DropTargetDelegate
}

/**
 * Provides the hooks required to enable drag and drop behavior for a drag and drop compatible collection component.
 */
export function useDragAndDrop(options: DragAndDropOptions): DragAndDrop {
    const dragAndDropHooks = useMemo(() => {
        const {
            onDrop,
            onInsert,
            onItemDrop,
            onReorder,
            onRootDrop,
            getItems,
            renderDragPreview,
            renderDropIndicator,
            dropTargetDelegate,
        } = options

        const isDraggable = !!getItems
        const isDroppable = !!(onDrop || onInsert || onItemDrop || onReorder || onRootDrop)

        const hooks = {} as DragAndDropHooks

        if (isDraggable) {
            hooks.drag = {
                useDraggableCollectionState: function useDraggableCollectionStateOverride(
                    props: DraggableCollectionStateOpts,
                ) {
                    return useDraggableCollectionState({
                        ...props,
                        ...options,
                    } as DraggableCollectionStateOptions)
                },
                useDraggableCollection: useDraggableCollection,
                useDraggableItem: useDraggableItem,
                renderPreview: renderDragPreview,
                Preview: DragPreview,
            }
        }

        if (isDroppable) {
            hooks.drop = {
                useDroppableCollectionState: function useDroppableCollectionStateOverride(
                    props: DroppableCollectionStateOptions,
                ) {
                    return useDroppableCollectionState({ ...props, ...options })
                },
                useDroppableItem: useDroppableItem,
                useDroppableCollection: function useDroppableCollectionOverride(
                    props: DroppableCollectionOptions,
                    state: DroppableCollectionState,
                    ref: RefObject<HTMLElement>,
                ) {
                    return useDroppableCollection({ ...props, ...options }, state, ref)
                },
                useDropIndicator: useDropIndicator,
                renderDropIndicator: renderDropIndicator,
                dropTargetDelegate: dropTargetDelegate,
                ListDropTargetDelegate: ListDropTargetDelegate,
            }
        }

        return hooks
    }, [options])

    return {
        dragAndDropHooks,
    }
}

export interface DropIndicatorRenderProps extends StyleProps {
    /**
     * Whether the drop indicator is currently the active drop target.
     * @selector [data-drop-target]
     */
    isDropTarget: boolean
}

export interface DropIndicatorProps
    extends AriaDropIndicatorProps,
        RenderProps<DropIndicatorRenderProps> {
    ref?: RefObject<HTMLElement>
}

/**
 * A DropIndicator is rendered between items in a collection to indicate where dropped data will be inserted.
 */
export const DropIndicator = (props: DropIndicatorProps) => {
    const { render } = useDropIndicatorContext()

    return render(props)
}

///
///
///
export interface DragAndDropContextValue {
    dragAndDropHooks?: DragAndDropHooks | undefined
    dragState?: DraggableCollectionState | undefined
    dropState?: DroppableCollectionState | undefined
}

export const [useDragAndDropContext, DragAndDropContext] =
    createGenericContext<DragAndDropContextValue>()

interface DropIndicatorContextValue {
    render: (props: DropIndicatorProps) => ReactNode
}

export const [useDropIndicatorContext, DropIndicatorContext] =
    createGenericContext<DropIndicatorContextValue>()

export const useRenderDropIndicator = (
    dragAndDropHooks?: DragAndDropHooks,
    dropState?: DroppableCollectionState,
) => {
    const renderDropIndicator = dragAndDropHooks?.drop?.renderDropIndicator
    const isVirtualDragging = false // dragAndDropHooks?.isVirtualDragging?.()
    const fn = useCallback(
        (target: ItemDropTarget) => {
            // Only show drop indicators when virtual dragging or this is the current drop target.
            if (isVirtualDragging || dropState?.isDropTarget(target)) {
                return renderDropIndicator ? (
                    renderDropIndicator(target)
                ) : (
                    <DropIndicator target={target} />
                )
            }

            return null
            // We invalidate whenever the target changes.
            // eslint-disable-next-line react-hooks/exhaustive-deps
        },
        [dropState?.target, isVirtualDragging, renderDropIndicator],
    )

    return dragAndDropHooks?.drop?.useDropIndicator ? fn : undefined
}
