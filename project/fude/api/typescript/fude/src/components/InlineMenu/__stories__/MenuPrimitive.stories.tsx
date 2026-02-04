import type { Meta, StoryObj } from "@storybook/react"

import { Button } from "../../Button/Button.js"
import { Popover } from "../../Popover/Popover.js"
import { Menu, MenuItem, MenuTrigger } from "../InlineMenu.js"

const meta = {
    title: "Components/Navigation/Menu/Primitivex",
    component: Menu,
    tags: ["autodocs"],
    /*
    argTypes: {
        variant: {
            control: { type: "inline-radio" },
            options: ["bar", "pill", "underlined", "ghost", "bordered"],
        },
        size: {
            control: { type: "inline-radio" },
            options: ["sm", "md", "lg"],
        },
    },
    */
} satisfies Meta<typeof Menu>

export default meta

type Story = StoryObj<typeof meta>

export const Basic: Story = {
    render: (props) => (
        <Menu>
            <MenuItem>A</MenuItem>
            <MenuItem>B</MenuItem>
        </Menu>
    ),
}

export const WithTrigger: Story = {
    render: (props) => (
        <MenuTrigger>
            <Button>Open Menu</Button>
            <Popover>
                <Menu>
                    <MenuItem>First Item</MenuItem>
                    <MenuItem>Second Item</MenuItem>
                    <MenuItem>Third Item</MenuItem>
                </Menu>
            </Popover>
        </MenuTrigger>
    ),
}
