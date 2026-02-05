import { Checkerboard } from "../Checkerboard"
import { Sketch } from "../ColorPicker"

const meta = {
    title: "Data Entry/ColorPicker",
    component: Checkerboard,
    tags: ["autodocs"],
} satisfies Meta<typeof Checkerboard>

export default meta

type Story = StoryObj<typeof meta>

export const Basic = { render: (props) => <Sketch color="#ff0000" /> }
