import type { StoryObj } from "@storybook/react-vite"
import { Ripple } from "./Ripple"
import { useRipple } from "./Ripple.hook"

const meta = {
    component: Ripple,
}
export default meta

type Story = StoryObj<typeof meta>

export const Overview: Story = () => {
    const { ripples, rippleProps, clearRipple } = useRipple()

    return (
        <div style={{ height: "250px", width: "250px" }} {...rippleProps}>
            <Ripple ripples={ripples} clearRipple={clearRipple} />
        </div>
    )
}
