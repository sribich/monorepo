import { filterDOMProps, useResizeObserver } from "@react-aria/utils"
import type { GridNode } from "@react-types/grid"
import type {
    AriaLabelingProps,
    DragPreviewRenderer,
    SelectionBehavior,
    SelectionMode,
} from "@react-types/shared"
import type { TableProps as AriaTableProps, ColumnSize } from "@react-types/table"
import {
    Children,
    type PropsWithRef,
    type ReactNode,
    type RefObject,
    use,
    useCallback,
    useEffect,
    useLayoutEffect,
    useMemo,
    useRef,
    useState,
} from "react"
import {
    type DroppableCollectionResult,
    FocusScope,
    type Key,
    ListKeyboardDelegate,
    useFocusRing,
    useHover,
    useLocale,
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
    type DraggableCollectionState,
    type DroppableCollectionState,
    type MultipleSelectionState,
    type TableColumnResizeState,
    type TableState,
    useMultipleSelectionState,
    useTableColumnResizeState,
    useTableState,
} from "react-stately"

import {
    createControlledContext,
    createGenericContext,
    createNewGenericContext,
} from "../../hooks/context"
import {
    DragAndDropContext,
    type DragAndDropHooks,
    DropIndicatorContext,
    type DropIndicatorProps,
    useDragAndDropContext,
} from "../../hooks/useDragAndDrop"
import { useObjectRef } from "../../hooks/useObjectRef"
import { useRenderProps, useStyleProps } from "../../hooks/useRenderProps"
import { useStyles } from "../../theme/props"
import {
    type CollectionProps,
    createBranchComponent,
    createCollectionComponent,
    useCachedChildren,
    useCollection,
    useCollectionChildren,
    useCollectionNode,
    useRenderedCollection,
} from "../../utils/collection/hooks"
import { MultiProvider } from "../MultiProvider"
import { mergeProps } from "../../utils/mergeProps"
import type { RenderProps, StyleProps, StyleRenderProps } from "../../utils/props"
import { CheckboxContext } from "../Checkbox/Checkbox"
import { TableStyleProvider, tableStyles, useTableStyles } from "./Table.styles"
import { TableCollection } from "./collection"
import { CollectionBuilder, CollectionItems } from "../../utils/collection/components"
import { CollectionRenderer, defaultCollectionRender } from "../../utils/collection/context"

////////////////////////////////////////////////////////////////////////////////
/// Common Context
////////////////////////////////////////////////////////////////////////////////
const TableStateContext = createNewGenericContext<TableState<unknown>>()

////////////////////////////////////////////////////////////////////////////////
/// TableResizingContext
////////////////////////////////////////////////////////////////////////////////
const [useTableColumnResizeStateContext, TableColumnResizeStateProvider] =
    createGenericContext<TableColumnResizeState<unknown> | null>(true)
const [useTableResizerContext, TableResizerProvider] =
    createGenericContext<TableResizerContext>(true)

interface TableResizerContext {
    tableWidth: number
    useTableColumnResizeState: typeof useTableColumnResizeState
    onResizeStart?: ((widths: Map<Key, ColumnSize>) => void) | undefined
    onResize?: ((widths: Map<Key, ColumnSize>) => void) | undefined
    onResizeEnd?: ((widths: Map<Key, ColumnSize>) => void) | undefined
}

export interface TableResizerProps extends StyleProps {
    ref?: RefObject<HTMLDivElement>
    /**
     * The table that the resizer is wrapping.
     */
    children?: ReactNode
    /**
     * Resize handler that is called when a user starts resizing a column.
     */
    onResizeStart?: (widths: Map<Key, ColumnSize>) => void
    /**
     * Resize handler that is called for every change in a column's size.
     */
    onResize?: (widths: Map<Key, ColumnSize>) => void
    /**
     * Resize handler that is called when a user stops resizing a column.
     */
    onResizeEnd?: (widths: Map<Key, ColumnSize>) => void
}

export const TableResizer = (props: TableResizerProps) => {
    const [width, setWidth] = useState(0)
    const sizingRef = useObjectRef(props.ref)

    useResizeObserver({
        ref: sizingRef,
        onResize() {
            if ("clientWidth" in sizingRef.current) {
                setWidth(Number(sizingRef.current.clientWidth))
            }
        },
    })

    useLayoutEffect(() => {
        if ("clientWidth" in sizingRef.current) {
            setWidth(Number(sizingRef.current.clientWidth))
        }
    }, [sizingRef])

    const context = useMemo(
        () => ({
            tableWidth: width,
            useTableColumnResizeState,
            onResizeStart: props.onResizeStart,
            onResize: props.onResize,
            onResizeEnd: props.onResizeEnd,
        }),
        [width, props.onResizeStart, props.onResize, props.onResizeEnd],
    )

    const styleProps = useStyleProps(props, {})

    return (
        <div {...styleProps} ref={sizingRef}>
            <TableResizerProvider value={context}>{props.children}</TableResizerProvider>
        </div>
    )
}

///=============================================================================
/// Table
///=============================================================================
const [useTableContext, TableContext] = createControlledContext<
    TableProps<unknown>,
    HTMLTableElement
>()
export { TableContext }

const TableConfigContext = createNewGenericContext<TableConfigContext>()

export interface TableConfigContext {
    /**
     * Whether the table allows rows to be dragged.
     */
    allowsDragging: boolean
    /**
     * Whether the table allows nothing to be selected.
     */
    disallowEmptySelection: boolean
    /**
     * The selection behavior of rows. This will be null if `selectionMode` is `none`.
     */
    selectionBehavior: SelectionBehavior | null
    /**
     * The type of row selection for the table.
     */
    selectionMode: SelectionMode
}

export namespace Table {
    export interface Props<T>
        extends Omit<AriaTableProps<T>, "children">,
            AriaLabelingProps,
            StyleRenderProps<TableRenderProps> {
        ref?: RefObject<HTMLTableElement>

        children?: ReactNode
        /**
         * How multiple selection should behave in the collection.
         */
        selectionBehavior?: SelectionBehavior
        /**
         * Whether `disabledKeys` applies to all interactions, or only selection.
         * @default "selection"
         */
        // disabledBehavior?: DisabledBehavior,
        /** Handler that is called when a user performs an action on the row. */
        // onRowAction?: (key: Key) => void,
        /** Handler that is called when a user performs an action on the cell. */
        onCellAction?: (key: Key) => void
        /** The drag and drop hooks returned by `useDragAndDrop` used to enable drag and drop behavior for the Table. */
        dragAndDropHooks?: DragAndDropHooks

        // TODO: renderEmptyState
    }

    export interface RenderProps {
        /**
         * Whether the table is currently the active drop target. This is only
         * true when the table itself is the root drop target, and not a child
         * drop target.
         */
        isDropTarget: boolean
        /**
         * Whether the table is currently focused by any means.
         */
        isFocused: boolean
        /**
         * Whether the table is currently focused via keyboard.
         */
        isFocusVisible: boolean
        /**
         * The state of the table.
         */
        state: TableState<unknown>
    }
}

export const Table = <T,>(props: Table.Props<T>) => {
    console.log("rendering a table?")
    const selectionState = useMultipleSelectionState(props)

    const { disallowEmptySelection, selectionBehavior, selectionMode } = selectionState

    const configContext = useMemo(
        () => ({
            allowsDragging: !!props.dragAndDropHooks?.drag,
            disallowEmptySelection,
            selectionBehavior: selectionMode === "none" ? null : selectionBehavior,
            selectionMode,
        }),
        [selectionBehavior, selectionMode, disallowEmptySelection, props.dragAndDropHooks],
    )

    const items = (
        <TableConfigContext value={configContext}>
            <CollectionItems {...props} />
        </TableConfigContext>
    )

    return (
        <CollectionBuilder items={items} initialCollection={() => new TableCollection<T>()}>
            {(collection) => (
                <TableView props={props} collection={collection} selectionState={selectionState} />
            )}
        </CollectionBuilder>
    )
}

///=============================================================================
/// TableView
///=============================================================================
namespace TableView {
    export interface Props<T> {
        props: Table.Props<T>
        collection: TableCollection<T>
        selectionState: MultipleSelectionState
    }
}

export const TableView = <T extends object>({
    props,
    collection,
    selectionState,
}: TableView.Props<T>) => {
    const ref = useObjectRef(props.ref)

    const state = useTableState({
        ...props,
        collection,
        children: undefined as never,
        UNSAFE_selectionState: props.selectionState,
    })

    const { gridProps } = useTable(props, state, ref)

    const { CollectionRoot } = use(CollectionRenderer)

    const { focusProps, isFocused, isFocusVisible } = useFocusRing()

    // Drag & Drop
    const { dragAndDropHooks } = props

    const selectionManager = state.selectionManager

    //
    const { selectionBehavior, selectionMode, disallowEmptySelection } = state.selectionManager

    const styles = useStyles(tableStyles, {})
    const styleProps = useStyleProps(props, {
        // isDropTarget: isRootDropTarget,
        isFocused,
        isFocusVisible,
        state,
    })

    // // Column Resizing
    // const resizingContext = useTableResizerContext()
    // let layoutState: TableColumnResizeState<unknown> | null = null
    //
    // if (resizingContext) {
    //     layoutState = resizingContext.useTableColumnResizeState(
    //         {
    //             tableWidth: resizingContext.tableWidth,
    //         },
    //         state,
    //     )
    //     styleProps.style ??= {}
    //     styleProps.style.tableLayout = "fixed"
    //     styleProps.style.width = "fit-content"
    // }

    const TableElement = useTableElement("table")
    console.log(collection)
    return (
        <MultiProvider
            values={[
                [TableStateContext, state],
                [TableStyleProvider, styles],
                // [TableColumnResizeStateProvider, layoutState],
                // [DragAndDropContext, { dragAndDropHooks, dragState, dropState }],
                // [DropIndicatorContext, { render: TableDropIndicator }],
            ]}
        >
            <FocusScope>
                <TableElement
                    {...mergeProps(
                        filterDOMProps(props),
                        gridProps,
                        focusProps,
                        // droppableCollection?.collectionProps,
                        styles.styles.table(),
                        styleProps,
                    )}
                    ref={ref}
                >
                    <CollectionRoot collection={collection} />
                </TableElement>
            </FocusScope>
            {/*dragPreview*/}
        </MultiProvider>
    )
}

///=============================================================================
/// TableHeader
///=============================================================================
export namespace TableHeader {
    export interface Props<T> extends StyleProps {
        ref?: RefObject<HTMLTableSectionElement>

        /**
         * A list of table columns when using a dynamic collection.
         */
        columns?: Iterable<T> | undefined
        /**
         * A list of `Column` children, or a render function that accepts
         * the children passed into the `columns` prop.
         */
        children?: ReactNode | ((column: T) => ReactNode)
        /**
         * A list of dependencies that will cause the column property
         * cache to be invalidated, similar to how useEffect works.
         */
        dependencies?: unknown[]
    }
}

export const TableHeader = /*@__PURE__*/ createBranchComponent(
    "tableheader",
    <T extends object>(props: TableHeader.Props<T>) => {
        const collection = TableStateContext.useContext().collection

        const headerRows = useCachedChildren({
            items: collection.headerRows,
            children: useCallback((node: Node<unknown>) => {
                if (node.type === "headerrow") {
                    return <TableHeaderRow node={node} />
                }

                throw new Error(`Unknown table header node type: ${node.type}`)
            }, []),
        })

        const { rowGroupProps } = useTableRowGroup()
        const { hoverProps, isHovered } = useHover({
            // onHoverStart: props.onHoverStart,
            // onHoverChange: props.onHoverChange,
            // onHoverEnd: props.onHoverEnd
        })

        const { styles } = useTableStyles()
        // const styleProps = useStyleProps(headerProps, {})

        const HeaderElement = useTableElement("thead")

        return (
            <HeaderElement
                {...mergeProps(
                    filterDOMProps(props),
                    rowGroupProps,
                    hoverProps,
                    styles.head(),
                    // styleProps,
                )}
            >
                {headerRows}
            </HeaderElement>
        )
    },
    (props) => {
        return (
            <CollectionItems dependencies={props.dependencies} items={props.columns}>
                {props.children}
            </CollectionItems>
        )
    },
)

///=============================================================================
/// TableHeaderRow
///=============================================================================
const TableHeaderRow = <T,>({ node }: { node: GridNode<T> }) => {
    const state = TableStateContext.useContext()

    const { CollectionNode } = use(CollectionRenderer)

    const ref = useRef<HTMLTableRowElement>(null)
    const { rowProps } = useTableHeaderRow({ node }, state, ref)

    const { checkboxProps } = useTableSelectAllCheckbox(state)

    const { styles } = useTableStyles()

    const TrElement = useTableElement("tr")
    /*
                <CheckboxContext value={{ slots: { selectAll: checkboxProps } }}>
                {children}
            </CheckboxContext>
            */
    return (
        <TrElement {...mergeProps(rowProps, styles.headRow())} ref={ref}>
            <CollectionNode collection={state.collection} parent={node} />
        </TrElement>
    )
}

///=============================================================================
/// Column
///=============================================================================
export namespace Column {
    export interface Props<T> extends RenderProps<T> {
        ref?: RefObject<HTMLTableColElement>

        id: Key
        /**
         * The rendered content of the column when rendering a nested
         * header statically.
         */
        title?: ReactNode
        /**
         * A list of dynamic nested child columns.
         */
        childColumns?: Iterable<T>

        isRowHeader?: boolean
        isResizable?: boolean
        /**
         *
         */
        width?: ColumnSize | undefined
        minWidth?: ColumnSize | undefined
        maxWidth?: ColumnSize | undefined
        defaultWidth?: ColumnSize | undefined
    }
}

export const Column = createCollectionComponent(
    "column",
    <T extends object>(props: Column.Props<T>, node: GridNode<T>) => {
        const state = TableStateContext.useContext()

        const ref = useRef<HTMLTableHeaderCellElement>(node.props.ref)
        const { columnHeaderProps } = useTableColumnHeader({ node }, state, ref)

        const { hoverProps, isHovered } = useHover({})
        const { focusProps, isFocused, isFocusVisible } = useFocusRing()

        let isResizing = false
        const layoutState = useTableColumnResizeStateContext()
        const layoutStyle = {} as { width: number }
        if (layoutState) {
            isResizing = layoutState.resizingColumn === node.key
            layoutStyle.width = layoutState.getColumnWidth(node.key)
        }

        const { styles } = useTableStyles()

        return (
            <th
                {...mergeProps(columnHeaderProps, hoverProps, focusProps, styles.column(), {
                    style: layoutStyle,
                })}
                ref={ref}
                data-hovered={isHovered || undefined}
                data-focused={isFocused || undefined}
                data-focus-visible={isFocusVisible || undefined}
            >
                {props.children}
            </th>
        )
        /*
        <TableColumnProvider value={{ column: node, triggerRef: ref }}>
                    {Children.count(node.props.children) > 1 ? (
                        <div {...styles.cellFlexWrapper()}>{node.props.children}</div>
                    ) : (
                        node.props.children
                    )}
                </TableColumnProvider>
                */

        /*
        const render = useHeaderRendererContext()

        let childColumns

        if (typeof render === "function") {
            childColumns = render
        } else if (typeof props.children !== "function") {
            // TODO: I'm pretty sure this can cause errors when rendering children, causing
            //       duplicate effects to run when the children aren't statically defined
            //       columns, but instead other content.
            childColumns = props.children
        }

        const children = useCollectionChildren({
            children: null,
            items: props.childColumns,
        })

        const renderedChildren = (
            <>
                {props.title ?? props.children}
                <div>foobar</div>
            </>
        )

        // TODO: Figure out a way to make this a little more robust. As it currently is,
        //       we can get into a state where we're not passing a title when it's required
        //       and cause issues.
        return useCollectionNode("column", props, props.ref, renderedChildren, children)
        */
    },
)

////////////////////////////////////////////////////////////////////////////////
/// ColumnResizer
////////////////////////////////////////////////////////////////////////////////
interface ColumnResizerProps {
    ref?: RefObject<HTMLDivElement>
}

export const ColumnResizer = (props: ColumnResizerProps) => {
    const layoutState = useTableColumnResizeStateContext()
    const resizerContext = useTableResizerContext()

    if (!layoutState || !resizerContext) {
        // TODO: Add a warning or error here because we're not wrapped
        return null
    }

    const { onResizeStart, onResize, onResizeEnd } = resizerContext
    const { column, triggerRef } = useTableColumnContext()

    const inputRef = useRef<HTMLInputElement>(null)
    const { resizerProps, inputProps, isResizing } = useTableColumnResize(
        {
            "aria-label": "tableResizer",
            column,
            onResizeStart: onResizeStart as (widths: Map<Key, ColumnSize>) => void,
            onResize: onResize as (widths: Map<Key, ColumnSize>) => void,
            onResizeEnd: onResizeEnd as (widths: Map<Key, ColumnSize>) => void,
            triggerRef,
        },
        layoutState,
        inputRef,
    )

    const { hoverProps, isHovered } = useHover({})
    const { focusProps, isFocused, isFocusVisible } = useFocusRing()

    /*
    const isEResizable = layoutState.getColumnMinWidth(column.key) >= layoutState.getColumnWidth(column.key)
    const isWResizable = layoutState.getColumnMaxWidth(column.key) <= layoutState.getColumnWidth(column.key)
    const { direction } = useLocale()
    let resizableDirection: ColumnResizerRenderProps["resizableDirection"] = "both"
    if (isEResizable) {
        resizableDirection = direction === "rtl" ? "right" : "left"
    } else if (isWResizable) {
        resizableDirection = direction === "rtl" ? "left" : "right"
    } else {
        resizableDirection = "both"
    }

    const objectRef = useObjectRef(ref)
    const [cursor, setCursor] = useState("")
    useEffect(() => {
        const style = window.getComputedStyle(objectRef.current)
        setCursor(style.cursor)
    }, [objectRef, resizableDirection])

    const [isMouseDown, setMouseDown] = useState(false)
    const onPointerDown = (e: PointerEvent) => {
        if (e.pointerType === "mouse") {
            setMouseDown(true)
        }
    }

    if (!isResizing && isMouseDown) {
        setMouseDown(false)
    }
    */

    const { styles } = useTableStyles()

    return (
        <div
            {...mergeProps(
                hoverProps,
                focusProps,
                resizerProps,
                /*{ onPointerDown },*/ styles.columnResizer(),
            )}
            // ref={objectRef}
            ref={props.ref}
            role="presentation"
            data-hovered={isHovered || undefined}
            data-focused={isFocused || undefined}
            data-focus-visible={isFocusVisible || undefined}
        >
            x
        </div>
    )
}

///=============================================================================
/// TableBody
///=============================================================================
export namespace TableBody {
    export interface Props<T> extends CollectionProps<T>, StyleProps {
        ref?: RefObject<HTMLTableSectionElement>
    }
}

export const TableBody = createBranchComponent(
    "tablebody",
    <T extends object>(props: TableBodyProps<T>) => {
        const state = TableStateContext.useContext()
        const collection = state.collection

        const { CollectionNode } = use(CollectionRenderer)

        const { rowGroupProps } = useTableRowGroup()

        //         const bodyProps = props.collection.body.props as TableBodyProps<unknown>
        //
        //
        //
        //         const children = useRenderedCollection(props.collection.rows, (node) => {
        //             switch (node.type) {
        //                 case "item":
        //                     return <TableBodyRow node={node} />
        //                 default:
        //                     throw new Error(`Unknown node type '${node.type}' in TableHeader.`)
        //             }
        //         })
        //
        // const { styles } = useTableStyles()
        // const styleProps = useStyleProps(bodyProps, {})

        const TbodyElement = useTableElement("tbody")

        return (
            <tbody
                {...mergeProps(rowGroupProps /*, styles.body(), styleProps*/)}
                // ref={props.collection.body.props.ref}
            >
                <CollectionNode collection={collection} parent={collection.body} />
            </tbody>
        )
    },
    (props) => {
        return (
            <CollectionItems dependencies={props.dependencies} items={props.columns}>
                {props.children}
            </CollectionItems>
        )
    },
)

///=============================================================================
/// Row
///=============================================================================
export interface RowProps {
    ref?: RefObject<HTMLTableRowElement>
    id?: Key
    children?: ReactNode
}

export const Row = createBranchComponent("item", (props: RowProps, node: GridNode<T>) => {
    const state = TableStateContext.useContext()

    const { CollectionNode } = use(CollectionRenderer)

    const ref = useRef<HTMLTableRowElement>(null)
    const { rowProps } = useTableHeaderRow({ node }, state, ref)

    const { checkboxProps } = useTableSelectAllCheckbox(state)

    const { styles } = useTableStyles()

    return (
        <>
            <tr {...mergeProps(rowProps, styles.row())} ref={ref}>
                <CollectionNode collection={state.collection} parent={node} />
            </tr>
        </>
    )
})

///=============================================================================
/// Cell
///=============================================================================
export namespace Cell {
    export interface Props {
        ref?: RefObject<HTMLTableDataCellElement>
        children?: ReactNode
    }
}

export const Cell = createCollectionComponent(
    "cell",
    (props: Cell.Props, node: GridNode<unknown>) => {
        const ref = useObjectRef(props.ref)
        const state = TableStateContext.useContext()

        node.column = state.collection.columns[node.index] as GridNode<T>

        const { gridCellProps, isPressed } = useTableCell({ node }, state, ref)

        const { hoverProps, isHovered } = useHover({})
        const { focusProps, isFocused, isFocusVisible } = useFocusRing()

        const { styles } = useTableStyles()
        const renderProps = useRenderProps(props, {})

        const TdElement = useTableElement("td")

        return (
            <TdElement
                {...mergeProps(
                    filterDOMProps(props),
                    gridCellProps,
                    hoverProps,
                    focusProps,
                    styles.cell(),
                    renderProps,
                )}
                ref={ref}
                data-hovered={isHovered || undefined}
                data-focused={isFocused || undefined}
                data-focus-visible={isFocusVisible || undefined}
                data-pressed={isPressed || undefined}
            >
                {/* Ensure children, such as select boxes, are not rendered with
                    the same collection renderer as the table itself.*/}
                <CollectionRenderer value={defaultCollectionRender}>
                    {renderProps.children}
                </CollectionRenderer>
            </TdElement>
        )
    },
)

////////////////////////////////////////////////////////////////////////////////
/// Drop Indicators
////////////////////////////////////////////////////////////////////////////////
const TableDropIndicator = (props: DropIndicatorProps) => {
    const ref = useObjectRef(props.ref)

    const { dragAndDropHooks, dropState } = useDragAndDropContext()

    if (!dragAndDropHooks?.drop || !dropState) {
        throw new Error(`Invariant`)
    }

    const buttonRef = useRef<HTMLDivElement>(null)
    const { dropIndicatorProps, isHidden, isDropTarget } = dragAndDropHooks.drop.useDropIndicator(
        props,
        dropState,
        buttonRef,
    )

    if (isHidden) {
        return null
    }

    return (
        <TableDropIndicatorView
            {...props}
            ref={ref}
            dropIndicatorProps={dropIndicatorProps}
            isDropTarget={isDropTarget}
            buttonRef={buttonRef}
        />
    )
}

interface TableDropIndicatorViewProps extends DropIndicatorProps {
    ref?: RefObject<HTMLElement>

    dropIndicatorProps: React.HTMLAttributes<HTMLElement>
    isDropTarget: boolean
    buttonRef: RefObject<HTMLDivElement>
}

const TableDropIndicatorView = (props: TableDropIndicatorViewProps) => {
    const { dropIndicatorProps, isDropTarget, buttonRef } = props

    const state = useTableStateContext()

    const { visuallyHiddenProps } = useVisuallyHidden()
    const renderProps = useRenderProps(props, { isDropTarget })

    return (
        <tr
            {...filterDOMProps(props as any)}
            {...renderProps}
            role="row"
            ref={props.ref as RefObject<HTMLTableRowElement>}
            data-drop-target={isDropTarget || undefined}
        >
            <td role="gridcell" colSpan={state.collection.columnCount} style={{ padding: 0 }}>
                <div
                    {...visuallyHiddenProps}
                    role="button"
                    {...dropIndicatorProps}
                    ref={buttonRef}
                />
            </td>
        </tr>
    )
}

/*
function RootDropIndicator() {
    const state = useContext(TableStateContext)!
    const { dragAndDropHooks, dropState } = useContext(DragAndDropContext)
    const ref = useRef<HTMLDivElement>(null)
    const { dropIndicatorProps } = dragAndDropHooks!.useDropIndicator!(
        {
            target: { type: "root" },
        },
        dropState!,
        ref,
    )
    const isDropTarget = dropState!.isDropTarget({ type: "root" })
    const { visuallyHiddenProps } = useVisuallyHidden()

    if (!isDropTarget && dropIndicatorProps["aria-hidden"]) {
        return null
    }

    return (
        <tr role="row" aria-hidden={dropIndicatorProps["aria-hidden"]} style={{ height: 0 }}>
            <td role="gridcell" colSpan={state.collection.columnCount} style={{ padding: 0 }}>
                <div role="button" {...visuallyHiddenProps} {...dropIndicatorProps} ref={ref} />
            </td>
        </tr>
    )
}
*/

// TODO: We need to handle non-div cases. If the table is not in a resize context
//       and is not virtual, use table items. Otherwise div.
const useTableElement = <T extends keyof JSX.IntrinsicElements>(element: T): T | "div" => {
    // return "div"
    return element
}

const useDnd = () => {
    let dragState: DraggableCollectionState | undefined = undefined
    let dropState: DroppableCollectionState | undefined = undefined
    let droppableCollection: DroppableCollectionResult | undefined = undefined
    let isRootDropTarget = false
    let dragPreview: React.JSX.Element | null = null
    const preview = useRef<DragPreviewRenderer>(null)

    if (dragAndDropHooks?.drag) {
        dragState = dragAndDropHooks.drag.useDraggableCollectionState({
            collection,
            selectionManager,
            preview,
        })
        dragAndDropHooks.drag.useDraggableCollection({}, dragState, ref)

        dragPreview = dragAndDropHooks.drag.renderPreview ? (
            <dragAndDropHooks.drag.Preview ref={preview}>
                {dragAndDropHooks.drag.renderPreview}
            </dragAndDropHooks.drag.Preview>
        ) : null
    }

    if (dragAndDropHooks?.drop) {
        dropState = dragAndDropHooks.drop.useDroppableCollectionState({
            collection,
            selectionManager,
        })

        const keyboardDelegate = new ListKeyboardDelegate({
            collection,
            disabledKeys: state.selectionManager.disabledKeys,
            disabledBehavior: state.selectionManager.disabledBehavior,
            ref,
        })

        const dropTargetDelegate =
            dragAndDropHooks.drop.dropTargetDelegate ||
            new dragAndDropHooks.drop.ListDropTargetDelegate(props.collection.rows, ref)

        droppableCollection = dragAndDropHooks.drop.useDroppableCollection(
            {
                keyboardDelegate,
                dropTargetDelegate,
            },
            dropState,
            ref,
        )
        isRootDropTarget = dropState.isDropTarget({ type: "root" })
    }
}
