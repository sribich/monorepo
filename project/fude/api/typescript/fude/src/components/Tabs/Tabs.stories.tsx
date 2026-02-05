import type { Meta, StoryObj } from "@storybook/react"
import { useListData } from "react-stately"

import { Tab, Tabs } from "./Tabs"

const meta = {
    title: "Navigation/Tabs",
    component: Tabs,
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
    args: {
        variant: "underline",
        size: "lg",
        radius: "md",
    },
} satisfies Meta<typeof Tabs>

export default meta

type Story = StoryObj<typeof meta>

export const Overview: Story = (props) => {
    const Wrap = () => {
        const list = useListData({
            initialItems: [
                { id: "foo", name: "My Account" },
                { id: "bar", name: "Company" },
                { id: "baz", name: "Team Members" },
            ],
        })

        return (
            <Tabs {...props} items={list.items}>
                {(item) => {
                    return (
                        <Tab {...item} title={item.name}>
                            <div>{item.name}</div>
                        </Tab>
                    )
                }}
            </Tabs>
        )
    }

    return <Wrap />
}

/*
<TabList>
                        {(item) => {
                            return <Tab id={item.id}>{item.name}</Tab>
                        }}
                    </TabList>
                    <TabPanel id="foo">a foo</TabPanel>
                    <TabPanel id="bar">a bar</TabPanel>
                    <TabPanel id="baz">a baz</TabPanel>
                    */

/*
<Tabs variant="underline" defaultSelectedKey={view} onSelectionChange={setView}>
    <TabList addons={<DatatableAddons />}>
        {views.map((view) => (
            <Tab
                key={view.uuid}
                id={view.uuid}
                icon={createElement(viewComponents[view.kind].icon, { size: 16, className: "mr-1" })}
            >
                {view.name}
            </Tab>
        ))}
    </TabList>
    {views.map((view) => (
        <TabPanel key={view.uuid} id={view.uuid}>
            <FilterBar />
            <View view={view} />
        </TabPanel>
    ))}
</Tabs>
*/

/*
export const Test: Story = {
    render: (props) => (
        <Tabs>
            <TabList>
                <Tab id="foo">foo</Tab>
                <Tab id="bar">bar</Tab>
            </TabList>
            <TabPanel id="foo">a foo</TabPanel>
            <TabPanel id="bar">a bar</TabPanel>
        </Tabs>
    ),
}
/*
export const Test2: Story = {
    render: (props) => (
        <Tabs>
            <TabList>
                <Tab id="foo">foo</Tab>
                <Tab id="bar">bar</Tab>
                <Tab id="baz">baz</Tab>
            </TabList>
            <TabPanel id="foo">a foo</TabPanel>
            <TabPanel id="bar">a bar</TabPanel>
            <TabPanel id="baz">a baz</TabPanel>
        </Tabs>
    ),
}
*/
