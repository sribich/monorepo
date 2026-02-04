import type { Collection as ICollection, Key, Node } from "@react-types/shared"
import type { Mutable } from "@sribich/ts-utils"
import type { ReactElement, ReactNode } from "react"

/**
 * An immutable object representing a node in a Collection.
 *
 * Nodes are built recursively when using the `useCollection`
 * family of hooks.
 *
 * A node includes the value of the item it represents, along
 * with metadata about its place in the collection.
 */
export class CollectionNode<T> implements Node<T> {
    readonly type: string
    readonly key: Key
    readonly value: T | null = null
    readonly level: number = 0
    readonly hasChildNodes: boolean = false
    readonly rendered: ReactNode = null
    readonly textValue: string = ""
    readonly "aria-label"?: string
    readonly index: number = 0
    readonly parentKey: Key | null = null
    readonly prevKey: Key | null = null
    readonly nextKey: Key | null = null
    readonly firstChildKey: Key | null = null
    readonly lastChildKey: Key | null = null
    readonly props: any = {}
    readonly render?: (node: Node<any>) => ReactElement

    constructor(type: string, key: Key) {
        this.type = type
        this.key = key
    }

    get childNodes(): Iterable<Node<T>> {
        throw new Error("childNodes is not implemented")
    }

    clone(): CollectionNode<T> {
        const node: Mutable<CollectionNode<T>> = new CollectionNode(this.type, this.key)

        node.value = this.value
        node.level = this.level
        node.hasChildNodes = this.hasChildNodes
        node.rendered = this.rendered
        node.textValue = this.textValue
        if (this["aria-label"]) {
            node["aria-label"] = this["aria-label"]
        }
        node.index = this.index
        node.parentKey = this.parentKey
        node.prevKey = this.prevKey
        node.nextKey = this.nextKey
        node.firstChildKey = this.firstChildKey
        node.lastChildKey = this.lastChildKey
        node.props = this.props
        if (this.render) {
            node.render = this.render
        }

        return node
    }
}

/**
 * An immutable Collection implementation. Updates are only allowed
 * when it is not marked as frozen.
 *
 * A collection is an immutable
 */
export class Collection<T> implements ICollection<Node<T>> {
    private keyMap: Map<Key, CollectionNode<T>> = new Map()
    private firstKey: Key | null = null
    private lastKey: Key | null = null
    private frozen = false

    get size() {
        return this.keyMap.size
    }

    getKeys() {
        return this.keyMap.keys()
    }

    isEmpty() {
        return this.size === 0
    }

    isPopulated() {
        return this.size > 0
    }

    *[Symbol.iterator]() {
        let node: Node<T> | undefined =
            this.firstKey != null ? this.keyMap.get(this.firstKey) : undefined

        while (node) {
            yield node
            node = node.nextKey != null ? this.keyMap.get(node.nextKey) : undefined
        }
    }

    getChildren(key: Key): Iterable<Node<T>> {
        const keyMap = this.keyMap

        return {
            *[Symbol.iterator]() {
                const parent = keyMap.get(key)
                let node = parent?.firstChildKey != null ? keyMap.get(parent.firstChildKey) : null

                while (node) {
                    yield node
                    node = node.nextKey != null ? keyMap.get(node.nextKey) : undefined
                }
            },
        }
    }

    getKeyBefore(key: Key) {
        let node = this.keyMap.get(key)

        if (!node) {
            return null
        }

        if (node.prevKey != null) {
            node = this.keyMap.get(node.prevKey)

            while (node && node.type !== "item" && node.lastChildKey != null) {
                node = this.keyMap.get(node.lastChildKey)
            }

            return node?.key ?? null
        }

        return node.parentKey
    }

    getKeyAfter(key: Key) {
        let node = this.keyMap.get(key)

        if (!node) {
            return null
        }

        if (node.type !== "item" && node.firstChildKey != null) {
            return node.firstChildKey
        }

        while (node) {
            if (node.nextKey != null) {
                return node.nextKey
            }

            if (node.parentKey != null) {
                node = this.keyMap.get(node.parentKey)
            } else {
                return null
            }
        }

        return null
    }

    getFirstKey() {
        return this.firstKey
    }

    getLastKey() {
        let node = this.lastKey != null ? this.keyMap.get(this.lastKey) : null

        while (node?.lastChildKey != null) {
            node = this.keyMap.get(node.lastChildKey)
        }

        return node?.key ?? null
    }

    getItem(key: Key): Node<T> | null {
        return this.keyMap.get(key) ?? null
    }

    at(): Node<T> {
        throw new Error("Not implemented")
    }

    clone(): this {
        // We need to clone using this.constructor so that subclasses have the right prototype.
        // TypeScript isn't happy about this yet.
        // https://github.com/microsoft/TypeScript/issues/3841
        const Constructor = this.constructor as { new (): Collection<T> }
        const collection: Collection<T> = new Constructor()

        collection.keyMap = new Map(this.keyMap)
        collection.firstKey = this.firstKey
        collection.lastKey = this.lastKey

        return collection as this
    }

    addNode(node: CollectionNode<T>) {
        if (this.frozen) {
            throw new Error("Cannot add a node to a frozen collection")
        }

        this.keyMap.set(node.key, node)
    }

    removeNode(key: Key) {
        if (this.frozen) {
            throw new Error("Cannot remove a node to a frozen collection")
        }

        this.keyMap.delete(key)
    }

    commit(firstKey: Key | null, lastKey: Key | null) {
        if (this.frozen) {
            throw new Error("Cannot commit a frozen collection")
        }

        this.firstKey = firstKey
        this.lastKey = lastKey
        this.frozen = true
    }
}
