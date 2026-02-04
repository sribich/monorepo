import type { Meta, StoryObj } from "@storybook/react"

import { Tab, Tabs } from "../Tabs"

const meta = {
    title: "Navigation/Tabs/Primitive",
    component: Tabs,
    tags: ["autodocs"],
    argTypes: {
        variant: {
            control: { type: "inline-radio" },
            options: ["underline", "bar", "pill", "ghost"],
        },
        size: {
            control: { type: "inline-radio" },
            options: ["sm", "md", "lg"],
        },
        /*
        color: {
            control: { type: "select" },
            options: ["default", "primary", "secondary", "success", "warning", "danger"],
        },
        */
        radius: {
            control: { type: "inline-radio" },
            options: ["none", "sm", "md", "lg", "full"],
        },
    },
} satisfies Meta<typeof Tabs>

export default meta

type Story = StoryObj<typeof meta>

export const Basic: Story = (props) => <Tabs {...props}></Tabs>

/*
<TabList>
                <Tab id="foo">foo</Tab>
                <Tab id="bar">bar</Tab>
            </TabList>
            <TabPanel id="foo">
                <div>afoo</div>
            </TabPanel>
            <TabPanel id="bar">
                <div>bbar</div>
            </TabPanel>
            */
