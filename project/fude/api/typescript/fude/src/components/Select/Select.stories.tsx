import type { Meta, StoryObj } from "@storybook/react-vite"
import { stylesToArgTypes } from "../../theme/props"
import { Select, SelectItem } from "./Select"
import { selectStyles } from "./Select.stylex"

const meta = {
    title: "Data Entry/Select",
    component: Select,
    tags: ["autodocs"],
    argTypes: {
        label: {
            control: { type: "text" },
        },
        labelPlacement: {
            control: { type: "select" },
            options: ["side", "top"],
        },
        size: {
            control: { type: "select" },
            options: ["xs", "sm", "md", "lg"],
        },
    },
    args: {
        label: "Label",
        labelPlacement: "top",
        size: "md",
    },
} satisfies Meta<typeof Select>

export default meta
type Story = StoryObj<typeof meta>

export const Overview = <T,>(props: Select.Props<T>) => (
    <Select {...props} label="Test">
        <SelectItem id="a">a</SelectItem>
        <SelectItem id="b">b</SelectItem>
        <SelectItem id="c">c</SelectItem>
    </Select>
)

export const RenderProps = <T,>(props: Select.Props<T>) => (
    <Select {...props} label="Test">
        <SelectItem id="a">a</SelectItem>
        <SelectItem id="b">b</SelectItem>
        <SelectItem id="c">c</SelectItem>
    </Select>
)

Overview.argTypes = {
    ...stylesToArgTypes(selectStyles),
}
