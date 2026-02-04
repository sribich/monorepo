import { Chip } from "../Chip.js"

const meta = {
    title: "Data Display/Chip",
    component: Chip,
    tags: ["autodocs"],
    argTypes: {
        color: {
            control: { type: "select" },
            options: ["default", "primary", "secondary", "success", "warning", "danger"],
        },
        radius: {
            control: { type: "select" },
            options: ["sm", "md", "full"],
        },
        size: {
            control: { type: "select" },
            options: ["sm", "md", "lg"],
        },
        variant: {
            control: { type: "select" },
            options: ["solid"],
        },
    },
} satisfies Meta<typeof Chip>

export default meta

type Story = StoryObj<typeof meta>

export const Basic = (props) => <Chip {...props}>Button Text</Chip>
