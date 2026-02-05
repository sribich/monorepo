import { PersonStanding } from "lucide-react"
import { fn, expect } from "storybook/test"

import preview from "@/preview"

import { stylesToArgTypes } from "../../theme/props.js"
import { Button, ButtonGroup } from "./Button"
import { buttonStyles } from "./Button.stylex"

const meta = preview.meta({
    component: Button,
    ...stylesToArgTypes(buttonStyles),
})

export const Size = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} size="xs">
            Extra Small
        </Button>
        <Button {...props} size="sm">
            Small
        </Button>
        <Button {...props} size="md">
            Medium
        </Button>
        <Button {...props} size="lg">
            Large
        </Button>
    </div>
))

export const Variant = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} variant="solid" color="primary">
            Solid
        </Button>
        <Button {...props} variant="ghost" color="primary">
            Ghost
        </Button>
        <Button {...props} variant="light" color="primary">
            Light
        </Button>
    </div>
))

export const Rounded = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} radius="none">
            None
        </Button>
        <Button {...props} radius="sm">
            Small
        </Button>
        <Button {...props} radius="md">
            Medium
        </Button>
        <Button {...props} radius="lg">
            Large
        </Button>
        <Button {...props} radius="full">
            Full
        </Button>
    </div>
))

export const Color = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} color="default">
            Default
        </Button>
        <Button {...props} color="primary">
            Primary
        </Button>
        <Button {...props} color="secondary">
            Secondary
        </Button>
        <Button {...props} color="success">
            Success
        </Button>
        <Button {...props} color="warning">
            Warning
        </Button>
        <Button {...props} color="danger" asChild>
            <div>Danger</div>
        </Button>
    </div>
))

export const Icon = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} startContent={<PersonStanding />}>
            Start Content
        </Button>
        <Button {...props} endContent={<PersonStanding />}>
            End Content
        </Button>
    </div>
))

export const IconOnly = meta.story((props) => (
    <div className="flex flex-row items-center gap-2">
        <Button {...props} iconOnly>
            <PersonStanding />
        </Button>
    </div>
))

export const Group = meta.story({
    render: (props) => (
        <ButtonGroup {...props}>
            <Button>First</Button>
            <Button isDisabled data-testid="2">
                Second
            </Button>
            <Button data-testid="3">Third</Button>
        </ButtonGroup>
    ),
    play: async ({ canvas, userEvent }) => {
        const button = await canvas.findAllByRole("button")

        await userEvent.click(button[0])
    },
})
