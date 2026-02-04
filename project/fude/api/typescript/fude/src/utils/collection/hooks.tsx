import type {
    CollectionBase as AriaCollectionBase,
    Key,
    SelectionBehavior,
    SelectionMode,
    ItemProps as SharedItemProps,
    SectionProps as SharedSectionProps,
} from "@react-types/shared"
import {
    type ReactElement,
    type ReactNode,
    type ReactPortal,
    type RefObject,
    cloneElement,
    createContext,
    useCallback,
    useMemo,
    useSyncExternalStore,
} from "react"
import { createPortal } from "react-dom"

import { createGenericContext } from "../../hooks/context"
import type { RenderChild, RenderProps } from "../props"
import { Document, ElementNode } from "./Document"
import type { Node } from "react-stately"
import { Collection } from "./Collection"

export interface CachedChildrenOptions<T> {
    items?: Iterable<T> | undefined
    children?: ReactNode | ((item: T) => ReactNode) | undefined
    dependencies?: unknown[] | undefined
    idScope?: Key | undefined
    addIdAndValue?: boolean | undefined
}

export function useCachedChildren<T extends object>(props: CachedChildrenOptions<T>): ReactNode {
    const { children, items, idScope, addIdAndValue, dependencies = [] } = props

    const cache = useMemo(() => new WeakMap(), dependencies)

    return useMemo(() => {
        if (items && typeof children === "function") {
            const res: ReactElement[] = []

            for (const item of items) {
                let rendered = cache.get(item)

                if (!rendered) {
                    rendered = children(item)

                    // @ts-ignore
                    let key = rendered.props.id ?? item.key ?? item.id

                    if (key == null) {
                        throw new Error("Could not determine key for item")
                    }

                    if (idScope) {
                        key = `${idScope}:${key}`
                    }

                    rendered = cloneElement(
                        rendered,
                        addIdAndValue ? { key, id: key, value: item } : { key },
                    )

                    cache.set(item, rendered)
                }

                res.push(rendered)
            }

            return res
        }

        if (typeof children !== "function") {
            return children
        }

        return null
    }, [children, items, cache, idScope, addIdAndValue])
}

export interface ItemRenderProps {
    /**
     * Whether the item is currently hovered with a mouse.
     * @selector [data-hovered]
     */
    isHovered: boolean
    /**
     * Whether the item is currently in a pressed state.
     * @selector [data-pressed]
     */
    isPressed: boolean
    /**
     * Whether the item is currently selected.
     * @selector [aria-selected=true]
     */
    isSelected: boolean
    /**
     * Whether the item is currently focused.
     * @selector [data-focused]
     */
    isFocused: boolean
    /**
     * Whether the item is currently keyboard focused.
     * @selector [data-focus-visible]
     */
    isFocusVisible: boolean
    /**
     * Whether the item is non-interactive, i.e. both selection and actions are disabled and the item may
     * not be focused. Dependent on `disabledKeys` and `disabledBehavior`.
     * @selector [aria-disabled]
     */
    isDisabled: boolean
    /** The type of selection that is allowed in the collection. */
    selectionMode: SelectionMode
    /** The selection behavior for the collection. */
    selectionBehavior: SelectionBehavior
    /**
     * Whether the item allows dragging.
     * @note This property is only available in collection components that support drag and drop.
     * @selector [draggable]
     */
    allowsDragging?: boolean
    /**
     * Whether the item is currently being dragged.
     * @note This property is only available in collection components that support drag and drop.
     * @selector [data-dragging]
     */
    isDragging?: boolean
    /**
     * Whether the item is currently an active drop target.
     * @note This property is only available in collection components that support drag and drop.
     * @selector [data-drop-target]
     */
}

export function useCollectionNode<T, TProps extends object>(
    Type: string,
    props: TProps,
    render: (node: Node<T>) => ReactElement,
    rendered?: ReactNode,
    children?: ReactNode,
) {
    const nodeRef = useCallback(
        (element: ElementNode<T> | null) => {
            element?.setProps(props, rendered, render)
        },
        [props, rendered, render],
    )

    // @ts-expect-error
    return <Type ref={nodeRef}>{children}</Type>
}

export interface ItemProps<T = object>
    extends Omit<SharedItemProps<T>, "children">,
        RenderProps<ItemRenderProps> {
    ref?: RefObject<HTMLElement>
    /**
     * The unique ID of the item, similar to "key".
     */
    id?: Key
    /**
     * The object value that this item represents. When using dynamic collections, this is set automatically.
     */
    value?: T
}

export interface SectionProps<T>
    extends Omit<SharedSectionProps<T>, "children" | "title"> /*, StyleProps*/ {
    ref?: RefObject<HTMLElement>
    /** The unique id of the section. */
    id?: Key
    /** The object value that this section represents. When using dynamic collections, this is set automatically. */
    value?: T
    /** Static child items or a function to render children. */
    children?: ReactNode | ((item: T) => ReactElement) | undefined
}

export const CollectionContext = createContext<CachedChildrenOptions<unknown> | null>(null)

export type DocumentKind<T> = Document<T>

export type AnyGeneric<T> = T
export type Collect<T> =
    T extends AnyGeneric<infer P> ? Omit<T, "children" | "items"> & CollectionProps<P> : never

export interface CollectionProps<T> extends Omit<AriaCollectionBase<T>, "children" | "items"> {
    /**
     * The content of the collection.
     *
     * When using a static collection, a ReactNode should be passed.
     * When using a dynamic collection, a render function should be passed.
     */
    children: ReactNode | ((item: T) => ReactNode)

    /**
     * The dynamic collection data.
     *
     * When items are passed, children should be a render function.
     */
    items?: Iterable<T>
    /**
     * A list of values that will invalidate the collection's cache.
     */
    dependencies?: unknown[]
}

export const [useCollectionScopeContext, CollectionScopeProvider] =
    createGenericContext<boolean>(true)

export const [useDocumentContext, DocumentProvider] = createGenericContext<Document<any, any>>(true)

/**
 * Creates a document that we can render into in order to build our collection.
 *
 * The document is subscribed to using `useSyncExternalStore`, so that updates
 * are dispatched to the component in a manner that makes sense for us.
 */
export const useCollectionDocument = <TItem extends object, TCollection extends Collection<TItem>>(
    initialCollection?: (() => TCollection) | undefined,
) => {
    const document = useMemo(
        () =>
            new Document<TItem, TCollection>(
                initialCollection?.() || (new Collection() as TCollection),
            ),
        [initialCollection],
    )

    const subscribe = useCallback((fn: () => void) => document.subscribe(fn), [document])
    const getSnapshot = useCallback(() => document.getCollection(), [document])

    const collection = useSyncExternalStore(subscribe, getSnapshot, getSnapshot)

    return { collection, document }
}

/**
 * TODO: Docs
 */
export const useCollectionPortal = <TItem extends object, TCollection extends Collection<TItem>>(
    props: CollectionProps<TItem>,
    document?: Document<TItem, TCollection>,
): ReactPortal => {
    const documentContext = useDocumentContext()
    const usingDocument = document ?? documentContext

    if (!usingDocument) {
        throw new Error(
            "useCollectionPortal was called without a document and outside of a DocumentProvider.",
        )
    }

    const children = useCollectionChildren(props)

    const scopedChildren = useMemo(
        () => <CollectionScopeProvider value={true}>{children}</CollectionScopeProvider>,
        [children],
    )

    return createPortal(scopedChildren, usingDocument as unknown as DocumentFragment)
}

/**
 * Creates a document and supporting portal to render into.
 *
 * @see useCollectionDocument
 * @see useCollectionPortal
 */
export const useCollection = <TItem extends object, TCollection extends Collection<TItem>>(
    props: CollectionProps<TItem>,
    initialCollection?: () => TCollection,
) => {
    const { collection, document } = useCollectionDocument<TItem, TCollection>(initialCollection)

    return {
        collection,
        portal: useCollectionPortal<TItem, TCollection>(props, document),
    }
}

/*
export const createCollectionComponent = <T extends object, P extends object>(
    type: string,
    render: (props: P, node?: Node<T>) => ReactNode,
): ((props: P) => ReactNode) => {
    const Component = ({ node }) => render(node.props, node.props.ref, node)

    const Result = (props: P) => {
        const inCollectionScope = useCollectionScopeContext()

        if (!inCollectionScope) {
            if (render.length >= 2) {
                throw new Error(`${render.name} cannot be rendered outside a collection`)
            }

            return render(props)
        }

        return useCollectionNode(type, props, props.children, (node) => <Component node={node} />)
    }

    Result.$$collectionNode = true

    // @ts-ignore
    Result.displayName = render.name
    return Result
}
*/

export function createCollectionComponent<T extends object, P extends { children?: ReactNode }>(
    type: string,
    render: (props: P) => ReactNode,
): (props: P) => ReactNode
export function createCollectionComponent<T extends object, P extends { children?: ReactNode }>(
    type: string,
    render: (props: P, node: Node<T>) => ReactNode,
): (props: P) => ReactNode
export function createCollectionComponent<T extends object, P extends { children: ReactNode }>(
    type: string,
    render: (props: P, node: Node<T>) => ReactNode,
    getChildren: (props: P) => ReactNode,
): (props: P) => ReactNode
export function createCollectionComponent<T extends object, P extends { children?: ReactNode }>(
    type: string,
    render: (props: P, node: Node<T>) => ReactNode,
    getChildren?: (props: P) => ReactNode,
): (props: P) => ReactNode {
    const Component = ({ node }: { node: Node<T> }) => render(node.props, node)

    const CollectionComponent = (props: P) => {
        // This is a node that contains nested collection items. Not all items may be
        // collection items, so we need to ask which items are collection items via
        // getChildren (this should also verify that items are as expected)
        if (getChildren) {
            return useCollectionNode<T, P>(
                type,
                props,
                (node) => <Component node={node} />,
                null,
                getChildren(props),
            )
        }

        const collectionScope = useCollectionScopeContext()

        if (!collectionScope) {
            if (render.length >= 2) {
                throw new Error(
                    `${render.name} cannot be rendered outside of a collection as it has a requirement on the collection node.`,
                )
            }

            return render(props, undefined as never)
        }

        return useCollectionNode<T, P>(
            type,
            props,
            (node) => <Component node={node} />,
            props.children,
            null,
        )
    }

    CollectionComponent.displayName = render.name

    return CollectionComponent
}

export function createBranchComponent<T extends object, P extends { children: any }>(
    type: string,
    render: (props: P, node: Node<T>) => ReactElement,
    getChildren: (props: P) => ReactNode = useCollectionChildren,
) {
    const Component = ({ node }: { node: Node<T> }) => render(node.props, node)

    const CollectionComponent = (props: P) => {
        return useCollectionNode<T, P>(
            type,
            props,
            (node) => <Component node={node} />,
            null,
            getChildren(props),
        )
    }

    CollectionComponent.displayName = render.name

    return CollectionComponent
}

export function useCollectionChildren<T extends object>(props: CachedChildrenOptions<T>) {
    return useCachedChildren({ ...props, addIdAndValue: true })
}

export const useRenderedCollection = <T extends object>(
    items: Iterable<T>,
    render: (child: T) => ReactNode,
) => {
    const cache = useMemo(() => new WeakMap(), [])

    return useMemo(() => {
        if (!items) {
            return null
        }

        const renderedItems = [] as ReactNode[]

        for (const item of items) {
            let rendered = cache.get(item)

            if (!rendered) {
                rendered = render(item)

                if (rendered.key == null) {
                    // @ts-ignore
                    const key = rendered.props.id ?? item.key ?? item.id

                    if (key == null) {
                        throw new Error(`Could not determine a key for collection child.`)
                    }

                    rendered = cloneElement(rendered, { key })
                }

                cache.set(item, rendered)
            }

            renderedItems.push(rendered)
        }

        return renderedItems
    }, [items])
}
