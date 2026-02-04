import type { Meta, StoryObj } from "@storybook/react"

import { Divider } from "./Divider"

const meta = {
    title: "Layout/Divider",
    component: Divider,
    tags: ["autodocs"],
    argTypes: {
        size: {
            control: "inline-radio",
            options: ["sm", "md", "lg"],
        },
    },
} satisfies Meta<typeof Divider>

export default meta

type Story = StoryObj<typeof meta>

export const Basic: Story = {
    render: (props) => <Divider {...props} className="h-4 w-36" />,
}
