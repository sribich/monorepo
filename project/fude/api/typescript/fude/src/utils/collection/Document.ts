import type { Node } from "@react-types/shared"
import type { Mutable } from "@sribich/ts-utils"
import type { ReactElement, ReactNode } from "react"
import { type Collection, CollectionNode } from "./Collection"

/**
 * A mutable node in the virtual DOM.
 *
 *
 *
 *
 * The BaseNode implements the logic required by react-dom to interact
 * with components in the portal that the Document renders into.
 *
 * When a node is mutated, the collection is marked as dirty in order
 * to consolodate the changes and cause a re-render.
 */
class BaseNode<TItem> {
    private _firstChild: ElementNode<TItem> | null = null
    private _lastChild: ElementNode<TItem> | null = null
    private _previousSibling: ElementNode<TItem> | null = null
    private _nextSibling: ElementNode<TItem> | null = null
    private _parentNode: BaseNode<TItem> | null = null

    ownerDocument: Document<TItem>

    constructor(ownerDocument: Document<TItem>) {
        // <any>
        this.ownerDocument = ownerDocument
    }

    *[Symbol.iterator]() {
        let node = this._firstChild

        while (node) {
            yield node
            node = node.nextSibling
        }
    }

    get firstChild() {
        return this._firstChild
    }

    set firstChild(firstChild) {
        this._firstChild = firstChild
        this.ownerDocument.markDirty(this)
    }

    get lastChild() {
        return this._lastChild
    }

    set lastChild(lastChild) {
        this._lastChild = lastChild
        this.ownerDocument.markDirty(this)
    }

    get previousSibling() {
        return this._previousSibling
    }

    set previousSibling(previousSibling) {
        this._previousSibling = previousSibling
        this.ownerDocument.markDirty(this)
    }

    get nextSibling() {
        return this._nextSibling
    }

    set nextSibling(nextSibling) {
        this._nextSibling = nextSibling
        this.ownerDocument.markDirty(this)
    }

    get parentNode() {
        return this._parentNode
    }

    set parentNode(parentNode) {
        this._parentNode = parentNode
        this.ownerDocument.markDirty(this)
    }

    appendChild(child: ElementNode<TItem>) {
        this.ownerDocument.startTransaction()

        if (child.parentNode) {
            child.parentNode.removeChild(child)
        }

        if (this.firstChild == null) {
            this.firstChild = child
        }

        if (this.lastChild) {
            this.lastChild.nextSibling = child
            child.index = this.lastChild.index + 1
            child.previousSibling = this.lastChild
        } else {
            child.previousSibling = null
            child.index = 0
        }

        child.parentNode = this
        child.nextSibling = null
        this.lastChild = child

        this.ownerDocument.markDirty(this)

        if (child.hasSetProps) {
            // Only add the node to the collection if we already received props for it.
            // Otherwise wait until then so we have the correct id for the node.
            this.ownerDocument.addNode(child)
        }

        this.ownerDocument.endTransaction()
        this.ownerDocument.queueUpdate()
    }

    insertBefore(newNode: ElementNode<TItem>, targetNode: ElementNode<TItem>) {
        if (targetNode == null) {
            return this.appendChild(newNode)
        }

        this.ownerDocument.startTransaction()

        if (newNode.parentNode) {
            newNode.parentNode.removeChild(newNode)
        }

        newNode.nextSibling = targetNode
        newNode.previousSibling = targetNode.previousSibling
        newNode.index = targetNode.index

        if (this.firstChild === targetNode) {
            this.firstChild = newNode
        } else if (targetNode.previousSibling) {
            targetNode.previousSibling.nextSibling = newNode
        }

        targetNode.previousSibling = newNode
        newNode.parentNode = targetNode.parentNode

        let node: ElementNode<TItem> | null = targetNode

        while (node) {
            node.index++
            node = node.nextSibling
        }

        if (newNode.hasSetProps) {
            this.ownerDocument.addNode(newNode)
        }

        this.ownerDocument.endTransaction()
        this.ownerDocument.queueUpdate()
    }

    removeChild(child: ElementNode<TItem>) {
        if (child.parentNode !== this || !this.ownerDocument.isMounted) {
            return
        }

        this.ownerDocument.startTransaction()

        let node = child.nextSibling

        while (node) {
            node.index--
            node = node.nextSibling
        }

        if (child.nextSibling) {
            child.nextSibling.previousSibling = child.previousSibling
        }

        if (child.previousSibling) {
            child.previousSibling.nextSibling = child.nextSibling
        }

        if (this.firstChild === child) {
            this.firstChild = child.nextSibling
        }

        if (this.lastChild === child) {
            this.lastChild = child.previousSibling
        }

        child.parentNode = null
        child.nextSibling = null
        child.previousSibling = null
        child.index = 0

        this.ownerDocument.removeNode(child)
        this.ownerDocument.endTransaction()
        this.ownerDocument.queueUpdate()
    }

    /**
     * We do not want to trigger any event listeners while we are rendering into
     * the portal.
     *
     * React requires us add the listeners or else it will cause errors, so let's
     * noop them.
     */
    addEventListener() {}

    /**
     * We do not want to trigger any event listeners while we are rendering into
     * the portal.
     *
     * React requires us add the listeners or else it will cause errors, so let's
     * noop them.
     */
    removeEventListener() {}
}

let id = 0

/**
 * A mutable element node in the fake DOM tree. It owns an immutable
 * Collection Node which is copied on write.
 */
export class ElementNode<T> extends BaseNode<T> {
    nodeType = 8 // COMMENT_NODE (we'd use ELEMENT_NODE but React DevTools will fail to get its dimensions)
    node: CollectionNode<T>
    private _index: number = 0
    hasSetProps = false

    constructor(type: string, ownerDocument: Document<T, any>) {
        super(ownerDocument)
        this.node = new CollectionNode(type, `react-aria-${++id}`)
        // Start a transaction so that no updates are emitted from the collection
        // until the props for this node are set. We don't know the real id for the
        // node until then, so we need to avoid emitting collections in an inconsistent state.
        this.ownerDocument.startTransaction()
    }

    get index() {
        return this._index
    }

    set index(index) {
        this._index = index
        this.ownerDocument.markDirty(this)
    }

    get level(): number {
        if (this.parentNode instanceof ElementNode) {
            return this.parentNode.level + (this.node.type === "item" ? 1 : 0)
        }

        return 0
    }

    updateNode() {
        const node = this.ownerDocument.getMutableNode(this)
        node.index = this.index
        node.level = this.level
        node.parentKey = this.parentNode instanceof ElementNode ? this.parentNode.node.key : null
        node.prevKey = this.previousSibling?.node.key ?? null
        node.nextKey = this.nextSibling?.node.key ?? null
        node.hasChildNodes = !!this.firstChild
        node.firstChildKey = this.firstChild?.node.key ?? null
        node.lastChildKey = this.lastChild?.node.key ?? null
    }

    setProps<T extends Element>(
        obj: any,
        rendered?: ReactNode,
        render?: (node: Node<any>) => ReactElement,
    ) {
        const node = this.ownerDocument.getMutableNode(this)
        const { value, textValue, id, ...props } = obj
        // props.ref = ref
        node.props = props
        node.rendered = rendered
        node.value = value
        if (render) {
            node.render = render
        }

        node.textValue =
            textValue || (typeof rendered === "string" ? rendered : "") || obj["aria-label"] || ""
        if (id != null && id !== node.key) {
            if (this.hasSetProps) {
                throw new Error("Cannot change the id of an item")
            }
            node.key = id
        }

        // If this is the first time props have been set, end the transaction started in the constructor
        // so this node can be emitted.
        if (!this.hasSetProps) {
            this.ownerDocument.addNode(this)
            this.ownerDocument.endTransaction()
            this.hasSetProps = true
        }

        this.ownerDocument.queueUpdate()
    }

    get style() {
        return {}
    }

    hasAttribute() {}
    setAttribute() {}
    setAttributeNS() {}
    removeAttribute() {}
}

/**
 * A mutable Document in the fake DOM. It owns an immutable Collection instance,
 * which is lazily copied on write during updates.
 *
 *
 * A mutable virtual document.
 *
 * A document owns a single `Collection` instance, which is written to through
 *
 * It is written to through a
 */
export class Document<
    TItem,
    TCollection extends Collection<TItem> = Collection<TItem>,
> extends BaseNode<TItem> {
    nodeType = 11 // DOCUMENT_FRAGMENT_NODE
    override ownerDocument = this
    dirtyNodes: Set<BaseNode<TItem>> = new Set()

    /**
     *
     */
    isMounted = true

    private collection: TCollection
    private collectionMutated: boolean
    private mutatedNodes: Set<ElementNode<TItem>> = new Set()
    private subscriptions: Set<() => void> = new Set()
    private transactionCount = 0

    constructor(collection: TCollection) {
        super(null as unknown as Document<TItem, TCollection>)

        this.collection = collection
        this.collectionMutated = true
    }

    createElement(type: string) {
        return new ElementNode(type, this)
    }

    createTextNode(value: string): never {
        throw new Error(
            `A text node containing '${value}' was rendered inside of a collection. Text nodes may not be rendered within a collection outside of a collection node wrapper.`,
        )
    }

    /**
     * Lazily gets a mutable instance of a Node. If the node has already
     * been cloned during this update cycle, it just returns the existing one.
     */
    getMutableNode(element: ElementNode<TItem>): Mutable<CollectionNode<TItem>> {
        let node = element.node
        if (!this.mutatedNodes.has(element)) {
            node = element.node.clone()
            this.mutatedNodes.add(element)
            element.node = node
        }
        this.markDirty(element)
        return node
    }

    private getMutableCollection() {
        if (!this.collectionMutated) {
            this.collection = this.collection.clone()
            this.collectionMutated = true
        }

        return this.collection
    }

    markDirty(node: BaseNode<TItem>) {
        this.dirtyNodes.add(node)
    }

    startTransaction() {
        this.transactionCount++
    }

    endTransaction() {
        this.transactionCount--
    }

    addNode(element: ElementNode<TItem>) {
        const collection = this.getMutableCollection()
        if (!collection.getItem(element.node.key)) {
            collection.addNode(element.node)

            for (const child of element) {
                this.addNode(child)
            }
        }

        this.markDirty(element)
    }

    removeNode(node: ElementNode<TItem>) {
        for (const child of node) {
            child.parentNode = null
            this.removeNode(child)
        }

        const collection = this.getMutableCollection()
        collection.removeNode(node.node.key)
        this.markDirty(node)
    }

    /**
     * Finalizes any outstanding collection update, updating all child nodes
     * and rendering them.
     */
    getCollection(): TCollection {
        // We do not want to render if any previous collection change is still
        // in the midst of reconceliation.
        if (this.transactionCount > 0) {
            return this.collection
        }

        for (const element of this.dirtyNodes) {
            if (element instanceof ElementNode && element.parentNode) {
                element.updateNode()
            }
        }

        this.dirtyNodes.clear()

        if (this.mutatedNodes.size) {
            const collection = this.getMutableCollection()

            for (const element of this.mutatedNodes) {
                if (element.parentNode) {
                    collection.addNode(element.node)
                }
            }

            collection.commit(this.firstChild?.node.key ?? null, this.lastChild?.node.key ?? null)
            this.mutatedNodes.clear()
        }

        this.collectionMutated = false
        return this.collection
    }

    /**
     * Calls the subscription callbacks when all transactions have
     * been finalized.
     *
     * We should have a callback feeding into a `useSyncExternalStore`
     * hook to coordinate updates with react.
     */
    queueUpdate() {
        // Do not update if we are inside of a nested transaction.
        if (this.dirtyNodes.size === 0 || this.transactionCount > 0) {
            return
        }

        for (const subscription of this.subscriptions) {
            subscription()
        }
    }

    /**
     * Adds a subscription which will be notified of any update to
     * the document.
     *
     * A subscription should be used to notify react that the virtual
     * DOM has been updated and needs to be re-rendered.
     *
     * A callback is returned which to remove the subscription when
     * invoked.
     */
    subscribe(subscription: () => void) {
        this.subscriptions.add(subscription)

        return () => this.subscriptions.delete(subscription)
    }
}
