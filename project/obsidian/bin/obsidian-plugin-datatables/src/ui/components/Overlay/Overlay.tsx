import { type StyleProps, createGenericContext, useComposedRefs } from "@sribich/fude"
import {
    type ElementRef,
    type MouseEvent,
    type ReactNode,
    type RefObject,
    useRef,
    useState,
} from "react"
import { createPortal } from "react-dom"

export interface OverlayContextValue {
    triggerRef: RefObject<ElementRef<"button">>
    open: boolean
    toggleOpen(): void
}

export const [useOverlayContext, OverlayProvider] = createGenericContext<OverlayContextValue>()

////////////////////////////////////////////////////////////////////////////////
/// Overlay
////////////////////////////////////////////////////////////////////////////////
export interface OverlayProps {
    children: ReactNode
    // useParentSize?: boolean
    // hideOnEnter?: boolean
}

export const Overlay = (props: OverlayProps) => {
    const triggerRef = useRef<ElementRef<"button">>(null)
    const [open, setOpen] = useState(false)

    return (
        <OverlayProvider value={{ triggerRef, open, toggleOpen: () => setOpen((prev) => !prev) }}>
            {props.children}
        </OverlayProvider>
    )

    /*

    const sizeRef = useRef<HTMLDivElement>(null)

    const hide = () => {
        setOpen(false)
    }

    return (
        <div className="flex flex-auto" ref={sizeRef} onClick={() => !open && setOpen(true)}>
            {open && (
                <OverlayImpl
                    sizeRef={sizeRef.current}
                    render={props.render}
                    hide={hide}
                    useParentSize={props.useParentSize ?? false}
                    hideOnEnter={props.hideOnEnter ?? false}
                />
            )}
            {props.children}
        </div>
    )
    */
}

////////////////////////////////////////////////////////////////////////////////
/// OverlayTrigger
////////////////////////////////////////////////////////////////////////////////
interface OverlayTriggerProps extends StyleProps {
    ref?: RefObject<HTMLElement>
    children: ReactNode
    onClick?: (event: MouseEvent) => boolean
}

export const OverlayTrigger = (props: OverlayTriggerProps) => {
    const context = useOverlayContext()

    const composedRefs = useComposedRefs(props.ref, context.triggerRef)

    if (context.open) {
        return null
    }

    const onClick = (event: MouseEvent) => {
        if (props.onClick?.(event)) {
            return
        }
        context.toggleOpen()
    }

    return (
        <div onClick={onClick} {...props} ref={composedRefs as any}>
            {props.children}
        </div>
    )
}

/*

const sizeRef = useRef<HTMLDivElement>(null)

const hide = () => {
    setOpen(false)
}

return (
    <div className="flex flex-auto" ref={sizeRef} onClick={() => !open && setOpen(true)}>
        {open && (
            <OverlayImpl
                sizeRef={sizeRef.current}
                render={props.render}
                hide={hide}
                useParentSize={props.useParentSize ?? false}
                hideOnEnter={props.hideOnEnter ?? false}
            />
        )}
        {props.children}
    </div>
)
*/

////////////////////////////////////////////////////////////////////////////////
/// OverlayContent
////////////////////////////////////////////////////////////////////////////////
interface OverlayContentProps extends StyleProps {
    ref?: RefObject<HTMLDivElement>
    children: ReactNode
}

export const OverlayContent = (props: OverlayContentProps) => {
    const context = useOverlayContext()

    const { top, left, height, width } = context.triggerRef.current?.getBoundingClientRect() ?? {
        top: 0,
        left: 0,
        height: 0,
        width: 0,
    }
    // useParentSize && sizeRef.parentElement
    //     ? sizeRef.parentElement.getBoundingClientRect()
    //     : sizeRef.getBoundingClientRect()

    if (!context.open) {
        return null
    }

    return createPortal(
        <div {...props}>
            <div className="overlay">
                <div
                    className="fixed left-0 top-0 z-10 h-screen w-screen cursor-default"
                    onClick={context.toggleOpen}
                />
                <div
                    className={`fixed z-20 min-w-fit`}
                    style={{ top, left, width, minHeight: height }}
                >
                    {props.children}
                </div>
            </div>
        </div>,
        document.body,
    )
}

////////////////////////////////////////////////////////////////////////////////
/// OverlayView
////////////////////////////////////////////////////////////////////////////////
/* import { FunctionComponent, useEffect } from "react"

import { createPortal } from "react-dom"

export type OverlayImplProps = {
    sizeRef: HTMLDivElement | null
    render: FunctionComponent<{ hide: () => void }>
    hide: () => void
    useParentSize: boolean
    hideOnEnter?: boolean
}

export const OverlayImpl = ({ sizeRef, render, hide, useParentSize, hideOnEnter }: OverlayImplProps) => {
    useEffect(() => {
        if (!sizeRef) {
            return
        }

        const eventListener = (event: KeyboardEvent) => {
            if (event.key === "Escape" || (hideOnEnter && event.key === "Enter")) {
                event.preventDefault()
                hide()
            }
        }

        document.addEventListener("keydown", eventListener)

        return () => document.removeEventListener("keydown", eventListener)
    }, [hide, sizeRef, hideOnEnter])

    if (!sizeRef) {
        return null
    }

    const { top, left, height, width } =
        useParentSize && sizeRef.parentElement
            ? sizeRef.parentElement.getBoundingClientRect()
            : sizeRef.getBoundingClientRect()

    return createPortal(
        <div className="overlay">
            <div className="fixed top-0 left-0 z-10 w-screen h-screen cursor-default" onClick={hide} />
            <div className={`fixed z-20 min-w-fit`} style={{ top, left, width, minHeight: height }}>
                {render({ hide })}
            </div>
        </div>,
        document.body,
    )
}
 */
