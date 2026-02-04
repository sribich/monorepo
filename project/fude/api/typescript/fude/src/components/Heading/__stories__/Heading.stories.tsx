import type { Meta, StoryObj } from "@storybook/react-vite"
import { HeadingOld } from "../Heading"

const meta = {
    title: "Text/Heading",
    component: HeadingOld,
} satisfies Meta<typeof HeadingOld>

export default meta

type Story = StoryObj<typeof meta>

export const Overview = (props) => <HeadingOld {...props}>Button Text</HeadingOld>
