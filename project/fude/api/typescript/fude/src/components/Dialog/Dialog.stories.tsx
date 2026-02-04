import type { Meta, StoryObj } from "@storybook/react"

import { Button } from "../Button/Button"
import { Dialog, DialogTrigger } from "./Dialog"
import { Popover } from "../Popover/Popover"

const meta: Meta<typeof Dialog> = {
    title: "Components/Overlays/Dialog",
    component: Dialog,
}
export default meta
type Story = StoryObj<typeof Dialog>

export const Primary: Story = {
    render: () => (
        <DialogTrigger>
            <Button>Open me</Button>
            <Popover>
                <Dialog>
                    <div>hello</div>
                </Dialog>
            </Popover>
        </DialogTrigger>
    ),
}
