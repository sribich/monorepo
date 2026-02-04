import { createContext, useContext, useMemo, type ReactElement, type ReactNode } from "react"
import { createPortal } from "react-dom"
import type { Collection } from "./Collection"
import {
    CollectionScopeProvider,
    DocumentProvider,
    useCollectionChildren,
    useCollectionDocument,
    useDocumentContext,
    type CachedChildrenOptions,
} from "./hooks"
import { createNewGenericContext } from "../../hooks/context"

///==============================================================================
/// CollectionRoot
///==============================================================================
export namespace CollectionBuilder {
    export interface Props<C extends Collection<object>> {
        items: ReactNode
        children: (collection: C) => ReactNode
        initialCollection?: () => C
    }
}

export const CollectionBuilder = <C extends Collection<V>, V extends object>(
    props: CollectionBuilder.Props<C>,
) => {
    const context = useDocumentContext()

    if (context) {
        return props.items
    }

    const { collection, document } = useCollectionDocument(props.initialCollection)

    return (
        <>
            <Hidden>
                <DocumentProvider value={document}>{props.items}</DocumentProvider>
            </Hidden>
            <CollectionInner render={props.children} collection={collection} />
        </>
    )

    // const { collection, document } = useCollectionDocument()
    // const { collection, portal } = useCollectionPortal(props, document)
}

export const HiddenContext = createContext<boolean>(false)

// Portal to nowhere
const hiddenFragment = typeof DocumentFragment !== "undefined" ? new DocumentFragment() : null

export function Hidden(props: { children: ReactNode }) {
    let isHidden = useContext(HiddenContext)

    if (isHidden) {
        // Don't hide again if we are already hidden.
        return <>{props.children}</>
    }

    let children = <HiddenContext.Provider value>{props.children}</HiddenContext.Provider>

    // In SSR, portals are not supported by React. Instead, render into a <template>
    // element, which the browser will never display to the user. In addition, the
    // content is not part of the DOM tree, so it won't affect ids or other accessibility attributes.

    return createPortal(children, hiddenFragment!)
}

// const Foo = createCollectionComponent("item", (props) => {})
// const Foo = createCollectionComponent("item", (props, node) => {})

const CollectionContext = createNewGenericContext<CachedChildrenOptions<unknown>>(true)

export const CollectionItems = <T extends object>(props: CachedChildrenOptions<T>) => {
    const context = CollectionContext.useContext()

    const dependencies = (context?.dependencies || []).concat(props.dependencies)
    const idScope = props.idScope || context?.idScope

    let children = useCollectionChildren({
        ...props,
        idScope,
        dependencies,
    })

    const document = useDocumentContext()

    if (document) {
        children = <CollectionRenderer>{children}</CollectionRenderer>
    }

    const childContext = useMemo(
        () => ({
            dependencies,
            idScope,
        }),
        [...dependencies, idScope],
    )

    return <CollectionContext value={childContext}>{children}</CollectionContext>
}

interface CollectionInnerProps<C extends object> {
    collection: C
    render: (collection: C) => ReactNode
}

const CollectionInner = <C extends object>({ collection, render }: CollectionInnerProps<C>) => {
    return render(collection)
}

interface CollectionRendererProps {
    children: ReactNode
}

const CollectionRenderer = ({ children }: CollectionRendererProps) => {
    const document = useDocumentContext()

    const wrappedChildren = (
        <DocumentProvider value={undefined}>
            <CollectionScopeProvider value={true}>{children}</CollectionScopeProvider>
        </DocumentProvider>
    )

    return createPortal(wrappedChildren, document as unknown as Element)
}
