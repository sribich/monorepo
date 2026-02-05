import type { Meta, StoryObj } from "@storybook/react-vite"
import { Heading } from "../Heading"

const meta = {
    title: "Text/Heading",
    component: Heading,
} satisfies Meta<typeof Heading>

export default meta

type Story = StoryObj<typeof meta>

export const Overview = (props) => <Heading {...props}>Button Text</Heading>

/*
export const Headers = meta.story(() => (
    <>
        <Heading level={1}>H1</Heading>
        <Heading level={2}>H2</Heading>
        <Heading level={3}>H3</Heading>
        <Heading level={4}>H4</Heading>
        <Heading level={5}>H5</Heading>
        <Heading level={6}>H6</Heading>
    </>
))
*/
