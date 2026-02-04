import type { Meta, StoryObj } from "@storybook/react"
import { ArrowLeft, ArrowRight } from "lucide-react"

// import { Section } from "../../../utils/collection/hooks.js"
import { Button } from "../../Button/Button.js"
import { Divider } from "../../Divider/Divider.js"
import { Header } from "../../Header/primitive/Header.js"
import { Popover } from "../../Popover/Popover.js"
import { Menu, MenuItem, MenuTrigger } from "../Menu.js"

const meta = {
    title: "Navigation/Menu",
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
            <MenuItem description="foo">A</MenuItem>
            <MenuItem description="bar">B</MenuItem>
            <Divider />
            <MenuItem>C</MenuItem>
            <MenuItem>D</MenuItem>
        </Menu>
    ),
}

export const WithTrigger: Story = {
    render: (props) => (
        <MenuTrigger>
            <Button>Open Menu</Button>
            <Menu>
                <MenuItem>A</MenuItem>
                <MenuItem>B</MenuItem>
            </Menu>
        </MenuTrigger>
    ),
}

export const DynamicItems: Story = {
    render: () => {
        const items = [
            { key: 1, label: "First Item" },
            { key: 2, label: "Second Item" },
            { key: 3, label: "Third Item" },
        ]
        return <Menu items={items}>{(item) => <MenuItem {...item}></MenuItem>}</Menu>
    },
}

// TODO: Make these gray out
export const DisabledKeys: Story = {
    render: () => {
        const items = [
            { key: 1, label: "First Item" },
            { key: 2, label: "Second Item" },
            { key: 3, label: "Third Item" },
        ]
        return (
            <Menu items={items} disabledKeys={[1, 3]}>
                {(item) => <MenuItem {...item}></MenuItem>}
            </Menu>
        )
    },
}

export const ActionEvent: Story = {
    render: (props) => <Menu></Menu>,
}

export const Variants: Story = {
    render: (props) => <Menu></Menu>,
}

export const SingleSelection: Story = {
    render: (props) => <Menu></Menu>,
}

export const MultipleSelection: Story = {
    render: (props) => <Menu></Menu>,
}

export const WithShortcut: Story = {
    // shortcut="⌘N"
    render: (props) => (
        <Menu>
            <MenuItem label="Cut" shortcut="ctrl+x" />
            <MenuItem label="Copy" shortcut="ctrl+c" />
            <MenuItem label="Paste" shortcut="ctrl+v" />
        </Menu>
    ),
}

/**
 * It is possible to add additional content to a MenuItem using the
 * `before` and `after` props.
 */
export const WithExtra: Story = {
    render: (props) => (
        <Menu>
            <MenuItem before={<ArrowLeft />}>Before</MenuItem>
            <MenuItem after={<ArrowRight />}>After</MenuItem>
        </Menu>
    ),
}

export const WithDescription: Story = {
    render: (props) => (
        <Menu>
            <MenuItem label="Cut" description="foobar" shortcut="ctrl+x" />
            <MenuItem label="Copy" description="foobar" shortcut="ctrl+c" />
            <MenuItem label="Paste" description="foobar" shortcut="ctrl+v" />
        </Menu>
    ),
}

export const WithSections: Story = {
    render: (props) => (
        <Menu>
            <MenuItem>A</MenuItem>
            <MenuItem>B</MenuItem>
            <Section>
                <Header>Section</Header>
                <MenuItem>C</MenuItem>
                <MenuItem>D</MenuItem>
            </Section>
        </Menu>
    ),
}

export const CustomTrigger: Story = {
    render: (props) => <Menu></Menu>,
}

export const Backdrop: Story = {
    render: (props) => <Menu></Menu>,
}

export const Routing: Story = {
    render: (props) => <Menu></Menu>,
}
