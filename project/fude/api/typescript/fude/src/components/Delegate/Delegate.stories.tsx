import type { Meta, StoryObj } from "@storybook/react"

import { Delegate } from "./Delegate.js"

const meta = {
    title: "Components/Interactive/Delegate",
    tags: ["autodocs"],
    argTypes: {
        asChild: {
            control: "boolean",
            description: "Test",
            table: {
                defaultValue: { summary: false },
                type: { summary: "boolean" },
            },
        },
    },
} satisfies Meta<{ asChild: boolean }>

export default meta

type Story = StoryObj<{ asChild: boolean }>

/**
 * A `Delegate` component merges its props with its next immediate
 * child, essentially giving the child component the functionality
 * of its parent.
 *
 * This is useful for many cases, such as having a button perform
 * navigation:
 *
 * ```tsx
 * <Button asChild>
 *   <Link to="/new/route" />
 * <Button/>
 * ```
 */
export const Basic: Story = {
    render: ({ asChild }) => {
        const Component = asChild ? Delegate : "button"

        return (
            <div className="[&>button]:h-8 [&>button]:gap-2 [&>button]:rounded-md [&>button]:border-2 [&>button]:px-4 [&>button]:text-sm">
                <Component>
                    <span>Hello</span>
                </Component>
            </div>
        )
    },
}
