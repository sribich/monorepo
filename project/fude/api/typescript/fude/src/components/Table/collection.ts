import { buildHeaderRows } from "@react-stately/table"
import type { GridNode } from "@react-types/grid"
import type { TableCollection as ITableCollection } from "@react-types/table"
import type { Key } from "react-aria"
import type { Node } from "react-stately"
import { Collection, CollectionNode } from "../../utils/collection/Collection"

export class TableCollection<T> extends Collection<T> implements ITableCollection<T> {
    headerRows: GridNode<T>[] = []
    columns: GridNode<T>[] = []
    rows: GridNode<T>[] = []
    rowHeaderColumnKeys: Set<Key> = new Set()
    head: CollectionNode<T> = new CollectionNode("tableheader", -1)
    body: CollectionNode<T> = new CollectionNode("tablebody", -2)
    columnsDirty = true

    override addNode(node: CollectionNode<T>) {
        super.addNode(node)

        this.columnsDirty ||= node.type === "column"

        if (node.type === "tableheader") {
            this.head = node
        } else if (node.type === "tablebody") {
            this.body = node
        }
    }

    override commit(firstKey: Key, lastKey: Key) {
        this.updateColumns()

        super.commit(firstKey, lastKey)

        this.rows = [...this.getChildren(this.body.key)]
    }

    private updateColumns() {
        if (!this.columnsDirty) {
            return
        }

        this.rowHeaderColumnKeys = new Set()
        this.columns = []

        const columnKeyMap = new Map()

        const visit = (node: Node<T>) => {
            switch (node.type) {
                case "column":
                    columnKeyMap.set(node.key, node)

                    if (!node.hasChildNodes) {
                        node.index = this.columns.length
                        this.columns.push(node)

                        if (node.props.isRowHeader) {
                            this.rowHeaderColumnKeys.add(node.key)
                        }
                    }
                    break
            }

            for (const child of this.getChildren(node.key)) {
                visit(child)
            }
        }

        for (const node of this.getChildren(this.head.key)) {
            visit(node)
        }

        this.headerRows = buildHeaderRows(columnKeyMap, this.columns)
        this.columnsDirty = false

        if (this.rowHeaderColumnKeys.size === 0 && this.columns.length > 0) {
            throw new Error(
                "A table must have at least one Column with the isRowHeader prop set to true",
            )
        }
    }

    get columnCount() {
        return this.columns.length
    }

    override *[Symbol.iterator]() {
        if (this.head.key === -1) {
            return
        }

        yield this.head
        yield this.body
    }

    override get size() {
        return this.rows.length
    }

    override getFirstKey() {
        return this.body.firstChildKey
    }

    override getLastKey() {
        return this.body.lastChildKey
    }

    override getKeyAfter(key: Key) {
        const node = this.getItem(key)

        if (node?.type === "column") {
            return node.nextKey ?? null
        }

        return super.getKeyAfter(key)
    }

    override getKeyBefore(key: Key) {
        const node = this.getItem(key)

        if (node?.type === "column") {
            return node.prevKey ?? null
        }

        const k = super.getKeyBefore(key)

        if (k != null && this.getItem(k)?.type === "tablebody") {
            return null
        }

        return k
    }

    override getChildren(key: Key): Iterable<Node<T>> {
        if (!this.getItem(key)) {
            for (const row of this.headerRows) {
                if (row.key === key) {
                    return row.childNodes
                }
            }
        }

        return super.getChildren(key)
    }

    override clone() {
        const collection = super.clone()

        collection.headerRows = this.headerRows
        collection.columns = this.columns
        collection.rowHeaderColumnKeys = this.rowHeaderColumnKeys
        collection.head = this.head
        collection.body = this.body

        return collection
    }

    getTextValue(key: Key): string {
        const row = this.getItem(key)

        if (!row) {
            return ""
        }

        // If the row has a textValue, use that.
        if (row.textValue) {
            return row.textValue
        }

        // Otherwise combine the text of each of the row header columns.
        const rowHeaderColumnKeys = this.rowHeaderColumnKeys
        const text: string[] = []

        for (const cell of this.getChildren(key)) {
            const column = this.columns[cell.index]
            if (rowHeaderColumnKeys.has(column.key) && cell.textValue) {
                text.push(cell.textValue)
            }

            if (text.length === rowHeaderColumnKeys.size) {
                break
            }
        }

        return text.join(" ")
    }
}
