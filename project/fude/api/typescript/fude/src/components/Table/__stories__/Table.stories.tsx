import type { Meta, StoryObj } from "@storybook/react-vite"
import {
    Cell,
    Column,
    ColumnResizer,
    Row,
    Table,
    TableBody,
    TableHeader,
    TableResizer,
} from "../Table"

const meta = {
    title: "Data Display/Table",
    component: Table,
    tags: ["autodocs"],
} satisfies Meta<typeof Table>

export default meta
type Story = StoryObj<typeof meta>

export const Default = (props) => (
    <Table>
        <TableHeader>
            <Column isRowHeader>A</Column>
            <Column>B</Column>
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
)

Default.meta = {
    iframe: false,
}

/*
<TableResizer onResize={console.log} onResizeStart={console.log}>
    </TableResizer>
*/
