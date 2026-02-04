import { fireEvent, render, screen } from "@testing-library/react"
import { describe, expect, it, vi } from "vitest"

import { Delegate } from "./Delegate.js"

describe("Delegate", () => {
    it("Handles listeners on the delegate", async () => {
        const onClick = vi.fn()

        render(
            <Delegate onClick={onClick} data-testid="delegate">
                <button />
            </Delegate>,
        )

        fireEvent.click(screen.getByTestId("delegate"))

        expect(onClick).toHaveBeenCalledOnce()
    })

    it("Handles listeners on the delegatee", async () => {
        const onClick = vi.fn()

        render(
            <Delegate data-testid="delegate">
                <button onClick={onClick} />
            </Delegate>,
        )

        fireEvent.click(screen.getByTestId("delegate"))

        expect(onClick).toHaveBeenCalledOnce()
    })

    it("Calls all event handlers", async () => {
        const delegator = vi.fn()
        const delegatee = vi.fn()

        render(
            <Delegate data-testid="delegate" onClick={delegator}>
                <button onClick={delegatee} />
            </Delegate>,
        )

        fireEvent.click(screen.getByTestId("delegate"))

        expect(delegator).toHaveBeenCalledOnce()
        expect(delegatee).toHaveBeenCalledOnce()
    })

    it("Can handle explicitly unset event listeners", async () => {
        const delegator = vi.fn()
        const delegatee = vi.fn()

        render(
            <>
                <Delegate data-testid="delegatee-unset" onClick={delegator}>
                    <button onClick={undefined} />
                </Delegate>
                <Delegate data-testid="delegator-unset" onClick={undefined}>
                    <button onClick={delegatee} />
                </Delegate>
            </>,
        )

        fireEvent.click(screen.getByTestId("delegatee-unset"))

        expect(delegator).toHaveBeenCalledOnce()
        expect(delegatee).toHaveBeenCalledTimes(0)

        fireEvent.click(screen.getByTestId("delegator-unset"))

        expect(delegator).toHaveBeenCalledOnce()
        expect(delegatee).toHaveBeenCalledOnce()
    })
})
