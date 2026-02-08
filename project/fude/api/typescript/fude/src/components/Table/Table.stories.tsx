import preview from "@/preview"

import {
    Cell,
    Column,
    ColumnResizer,
    ResizableTableContainer,
    Row,
    Table,
    TableBody,
    TableHeader,
} from "./Table"

const meta = preview.meta({
    title: "Data Display/Table",
    component: Table,
    tags: ["autodocs"],
})

export const Overview = meta.story({
    render: () => (
        <ResizableTableContainer>
        <Table>
            <TableHeader>
                <Column resizable defaultWidth={100} isRowHeader>A</Column>
                <Column resizable defaultWidth={100}>B</Column>
                <Column>C</Column>
            </TableHeader>
            <TableBody>
                <Row>
                    <Cell>A</Cell>
                    <Cell>B</Cell>
                    <Cell>C</Cell>
                </Row>
                <Row>
                    <Cell>D</Cell>
                    <Cell>E</Cell>
                    <Cell>F</Cell>
                </Row>
            </TableBody>
        </Table>
        </ResizableTableContainer>
    )
})
