import { createContext, type ComponentType, type ReactNode, type RefObject } from "react"
import type { Collection, Node } from "react-stately"
import { useCachedChildren } from "./hooks"
import type { ItemDropTarget } from "react-aria"
import type { Key, LayoutDelegate } from "@react-types/shared"

//==============================================================================
// OwnerDocumentContext
//==============================================================================
export const OwnerDocumentContext = createContext<Document>(window.document)

///==============================================================================
/// CollectionRoot
///==============================================================================
export namespace CollectionRoot {
    export interface Props {
        collection: Collection<Node<unknown>>
        /** A set of keys for items that should always be persisted in the DOM. */
        persistedKeys?: Set<Key> | null
        /** A ref to the scroll container for the collection. */
        scrollRef: RefObject<HTMLElement | null>
        renderDropIndicator?: ((target: ItemDropTarget) => ReactNode) | undefined
    }
}

export const CollectionRoot = (props: CollectionRoot.Props) => {
    return useCollectionRenderer(props.collection, null, props.renderDropIndicator)
}

///==============================================================================
/// CollectionNode
///==============================================================================
export namespace CollectionNode {
    export interface Props {
        collection: Collection<Node<unknown>>
        parent: Node<unknown>
        renderDropIndicator?: (target: ItemDropTarget) => ReactNode
    }
}

export const CollectionNode = (props: CollectionNode.Props) => {
    return useCollectionRenderer(props.collection, props.parent, props.renderDropIndicator)
}

///==============================================================================
/// CollectionRenderer
///==============================================================================
const useCollectionRenderer = (
    collection: Collection<Node<unknown>>,
    parent: Node<unknown> | null,
    renderDropIndicator?: ((target: ItemDropTarget) => ReactNode) | undefined,
) => {
    return useCachedChildren({
        items: (parent ? collection.getChildren?.(parent.key) : collection) ?? [],
        dependencies: [renderDropIndicator],
        children: (node) => {
            const rendered = node.render?.(node)

            if (!renderDropIndicator || node.type !== "item") {
                return rendered
            }

            const key = node.key
            const nextKey = collection.getKeyAfter(key)

            return (
                <>
                    {renderDropIndicator({ type: "item", key, dropPosition: "before" })}
                    {rendered}
                    {nextKey !== null &&
                        renderDropIndicator({ type: "item", key, dropPosition: "after" })}
                </>
            )
        },
    })
}

export interface CollectionRenderer {
    /**
     * Whether this collection is virtualized.
     */
    isVirtualized?: boolean
    layoutDelegate?: LayoutDelegate
    CollectionRoot: ComponentType<CollectionRoot.Props>
    CollectionNode: ComponentType<CollectionNode.Props>
}

export const defaultCollectionRender: CollectionRenderer = {
    CollectionRoot,
    CollectionNode,
} satisfies CollectionRenderer

export const CollectionRenderer = createContext(defaultCollectionRender)
