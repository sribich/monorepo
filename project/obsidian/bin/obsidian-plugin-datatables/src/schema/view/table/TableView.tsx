import { Cell, Column, ColumnResizer, Row, Table, TableBody, TableHeader, TableResizer } from "@sribich/fude"

import { useSchema } from "../../../ui/hooks/useSchema"
import { useViewScopeContext } from "../../../ui/hooks/useViewScope"
import { PropertyField } from "../../property/PropertyField"

export const TableView = () => {
    const schema = useSchema()

    const viewScope = useViewScopeContext()

    const onResizeEnd = (widths: Map<number | string, number | string>) => {
        schema.view.resizeColumns(viewScope.view, widths)
    }

    // TODO: Description for aria-label
    return (
        <TableResizer onResizeEnd={onResizeEnd}>
            <Table aria-label={`${viewScope.schema.name} table view`}>
                <TableHeader columns={viewScope.properties}>
                    {(column) => (
                        <Column
                            id={column.uuid}
                            isRowHeader
                            isResizable
                            defaultWidth={viewScope.schema.config.properties.find((it) => it.id === column.uuid)?.width}
                        >
                            {column.name}
                            <ColumnResizer />
                        </Column>
                    )}
                </TableHeader>
                <TableBody items={viewScope.filteredDocuments}>
                    {(row) => (
                        <Row id={row.path}>
                            {viewScope.properties.map((it) => {
                                return (
                                    <Cell>
                                        <PropertyField property={it} document={row} />
                                    </Cell>
                                )
                            })}
                        </Row>
                    )}
                </TableBody>
            </Table>
        </TableResizer>
    )
}
