import type {
    AriaLabelingProps,
    DragPreviewRenderer,
    SelectionBehavior,
    SelectionMode,
} from "@react-types/shared"
import {
    type DroppableCollectionResult,
    FocusScope,
    type Key,
    ListKeyboardDelegate,
    useFocusRing,
    useHover,
    useTable,
    useTableCell,
    useTableColumnHeader,
    useTableColumnResize,
    useTableHeaderRow,
    useTableRowGroup,
    useTableSelectAllCheckbox,
    useVisuallyHidden,
} from "react-aria"
import {
    Cell as AriaCell,
    type CellProps as AriaCellProps,
    Column as AriaColumn,
    type ColumnProps as AriaColumnProps,
    ColumnResizer as AriaColumnResizer,
    type ColumnResizerProps as AriaColumnResizerProps,
    Row as AriaRow,
    type RowProps as AriaRowProps,
    Table as AriaTable,
    TableBody as AriaTableBody,
    type TableBodyProps as AriaTableBodyProps,
    TableHeader as AriaTableHeader,
    type TableHeaderProps as AriaTableHeaderProps,
    type TableProps as AriaTableProps,
    ResizableTableContainer as AriaResizableTableContainer,
    type ResizableTableContainerProps as AriaResizableTableContainerProps,
    useRenderProps,
} from "react-aria-components"
import {
    type DraggableCollectionState,
    type DroppableCollectionState,
    type MultipleSelectionState,
    useTableState,
} from "react-stately"

import { createControlledContext, createGenericContext } from "../../hooks/context"
import { type CachedStyles, useStyles } from "../../theme/props"
import { mergeProps } from "../../utils/mergeProps"
import { tableStyles } from "./Table.stylex"
import type { ReactNode } from "react"

//==============================================================================
// Table Context
//==============================================================================
const [useTableContext, TableContext] = createControlledContext<Table.Props, HTMLTableElement>()
21
export const [useTableStyles, TableStyleProvider] =
    createGenericContext<CachedStyles<typeof tableStyles>>()

//==============================================================================
// TableResizer
//==============================================================================
export namespace ResizableTableContainer {
    export interface Props extends AriaResizableTableContainerProps {}
}

export const ResizableTableContainer = (props: ResizableTableContainer.Props) => {
    return <AriaResizableTableContainer {...props} />
}

//==============================================================================
// Table
//==============================================================================
export namespace Table {
    export interface Props extends AriaTableProps {}
}

export const Table = (props: Table.Props) => {
    const { styles, values } = useStyles(tableStyles, {})

    return (
        <TableStyleProvider value={{ styles, values }}>
            <AriaTable {...mergeProps(props, styles.table())} />
        </TableStyleProvider>
    )
}

//==============================================================================
// TableHeader
//==============================================================================
export namespace TableHeader {
    export interface Props<T> extends AriaTableHeaderProps<T> {}
}

export const TableHeader = <T extends object>(props: TableHeader.Props<T>) => {
    const { styles } = useTableStyles()

    // TODO: styles.headRow
    return <AriaTableHeader {...mergeProps(props, styles.head())} />
}

//==============================================================================
// Column
//==============================================================================
export namespace Column {
    export interface Props extends Omit<AriaColumnProps, "children"> {
        children: ReactNode
        resizable?: boolean | undefined
    }
}

export const Column = (props: Column.Props) => {
    const { styles } = useTableStyles()

    return (
        <AriaColumn {...mergeProps(props, styles.columnWrapper())}>
            <div {...styles.column()}>
                {props.children}
                {props.resizable && <ColumnResizer />}
            </div>
        </AriaColumn>
    )
}

//==============================================================================
// ColumnResizer
//==============================================================================
export namespace ColumnResizer {
    export interface Props extends AriaColumnResizerProps {}
}

export const ColumnResizer = (props: ColumnResizer.Props) => {
    const { styles } = useTableStyles()

    return <AriaColumnResizer {...mergeProps(props, styles.columnResizer())} />
}

//==============================================================================
// TableBody
//==============================================================================
export namespace TableBody {
    export interface Props<T> extends AriaTableBodyProps<T> {}
}

export const TableBody = <T extends object>(props: TableBody.Props<T>) => {
    const { styles } = useTableStyles()

    return <AriaTableBody {...mergeProps(props, styles.body())} />
}

//==============================================================================
// Row
//==============================================================================
export namespace Row {
    export interface Props<T> extends AriaRowProps<T> {}
}

export const Row = <T extends object>(props: Row.Props<T>) => {
    const { styles } = useTableStyles()

    return <AriaRow {...mergeProps(props, styles.row())} />
}

//==============================================================================
// Cell
//==============================================================================
export namespace Cell {
    export interface Props extends AriaCellProps {}
}

export const Cell = (props: Cell.Props) => {
    const { styles } = useTableStyles()

    return <AriaCell {...mergeProps(props, styles.cell())} />
}
