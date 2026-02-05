import { render } from "@testing-library/react"
import { type UserEvent, userEvent } from "@testing-library/user-event"
import { createRef } from "react"
import { beforeAll, describe, expect, it } from "vitest"

import { TabListPrimitive, TabPanelPrimitive, TabPrimitive, TabsPrimitive } from "../TabsPrimitive"

const testItems = new Array(100).fill(null).map((_, index) => ({
    id: `${index}`,
    title: `Item ${index}`,
    content: `Content ${index}`,
}))

describe("TabsPrimitive", () => {
    let user: UserEvent

    beforeAll(() => {
        user = userEvent.setup()
    })

    it("Should forward its ref", () => {
        const wrapperRef = createRef<HTMLDivElement>()
        const listRef = createRef<HTMLDivElement>()
        const itemRef = createRef<HTMLDivElement>()
        const panelRef = createRef<HTMLDivElement>()

        const { getByTestId } = render(
            <TabsPrimitive ref={wrapperRef}>
                <TabListPrimitive ref={listRef}>
                    <TabPrimitive id="1" ref={itemRef}>
                        Title
                    </TabPrimitive>
                </TabListPrimitive>
                <TabPanelPrimitive id="1" ref={panelRef}>
                    Content
                </TabPanelPrimitive>
            </TabsPrimitive>,
        )

        expect(wrapperRef).not.toBeNull()
        expect(listRef).not.toBeNull()
        expect(itemRef.current?.innerHTML).toEqual("Title")
        expect(panelRef.current?.innerHTML).toEqual("Content")
    })

    // TODO: https://www.w3.org/WAI/ARIA/apg/patterns/tabs/
    describe("Accessibility", () => {})

    describe("Functionality", () => {
        it("Should render", () => {
            const { getByTestId } = render(
                <TabsPrimitive data-testid="test">
                    <TabListPrimitive>
                        <TabPrimitive id="1">1</TabPrimitive>
                        <TabPrimitive id="2">2</TabPrimitive>
                        <TabPrimitive id="3">3</TabPrimitive>
                    </TabListPrimitive>
                    <TabPanelPrimitive id="1">1</TabPanelPrimitive>
                    <TabPanelPrimitive id="2">2</TabPanelPrimitive>
                    <TabPanelPrimitive id="3">3</TabPanelPrimitive>
                </TabsPrimitive>,
            )

            const container = getByTestId("test")

            expect(container.childElementCount).toEqual(2)
            expect(container.childNodes[0]).toHaveAttribute("role", "tablist")
            expect(container.childNodes[1]).toHaveAttribute("role", "tabpanel")
        })

        it("Should support collection items", () => {
            const { getByTestId } = render(
                <TabsPrimitive data-testid="test">
                    <TabListPrimitive items={testItems}>
                        {(item) => (
                            <TabPrimitive key={item.id} id={item.id}>
                                {item.title}
                            </TabPrimitive>
                        )}
                    </TabListPrimitive>
                    {testItems.map((item) => (
                        <TabPanelPrimitive key={item.id} id={item.id}>
                            {item.content}
                        </TabPanelPrimitive>
                    ))}
                </TabsPrimitive>,
            )

            const container = getByTestId("test")

            expect(container.childElementCount).toEqual(2)
            expect(container.childNodes[0]?.childNodes.length).toEqual(100)
        })

        it("Should support render props for items", () => {
            const { getByTestId } = render(
                <TabsPrimitive>
                    <TabListPrimitive>
                        <TabPrimitive id="1" data-testid="test">
                            {() => "RenderProps"}
                        </TabPrimitive>
                    </TabListPrimitive>
                    <TabPanelPrimitive id="1">Content</TabPanelPrimitive>
                </TabsPrimitive>,
            )

            const container = getByTestId("test")

            expect(container.innerHTML).toEqual("RenderProps")
        })

        it("Should support render props for panels", () => {
            const { getByTestId } = render(
                <TabsPrimitive>
                    <TabListPrimitive>
                        <TabPrimitive id="1">Title</TabPrimitive>
                    </TabListPrimitive>
                    <TabPanelPrimitive id="1" data-testid="test">
                        {() => "RenderProps"}
                    </TabPanelPrimitive>
                </TabsPrimitive>,
            )

            const container = getByTestId("test")

            expect(container.innerHTML).toEqual("RenderProps")
        })
    })
})

describe("Tabs", () => {})
