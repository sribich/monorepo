import { mergeProps } from "@react-aria/utils"
import {
    type Context,
    createContext,
    type ForwardedRef,
    type RefObject,
    use,
    useContext,
    useMemo,
} from "react"

import { mergeRefs } from "../utils/refs.js"
import { useObjectRef } from "./useObjectRef.js"

export interface SlottedContext<T> {
    slots?: Record<string | symbol, T>
}
export interface SlotProps {
    slot?: string | null | undefined
}

export type WithRef<TProps, TRef extends Element> = TProps & { ref?: ForwardedRef<TRef> }

export type ControlledContext<TProps, TRef extends Element> =
    | (SlottedContext<WithRef<TProps, TRef>> & Partial<WithRef<TProps, TRef>>)
    | WithRef<TProps, TRef>
    | null
    | undefined

type GenericContext<T> = (() => T) & { isProvided(): boolean }

export const DEFAULT_SLOT = Symbol("default")

export const createOptionalContext = <const T>(): readonly [
    GenericContext<T | undefined>,
    Context<T | undefined>,
] => {
    const context = createContext<T | undefined>(undefined)

    function useGenericContext() {
        const maybeContext = use(context)

        return maybeContext
    }

    useGenericContext.isProvided = () => {
        return !!use(context)
    }

    return [useGenericContext, context] as const
}

export function createGenericContext<const T>(
    nullable: true,
): readonly [GenericContext<T | undefined>, Context<T | undefined>]
export function createGenericContext<const T>(
    nullable?: false,
): readonly [GenericContext<T>, Context<T>]
export function createGenericContext<const T>(nullable?: boolean) {
    const context = createContext<T | undefined>(undefined)

    function useGenericContext() {
        const maybeContext = use(context)

        if (!maybeContext && !nullable) {
            throw new Error(`useGenericContext must be used in scope of its provider`)
        }

        return maybeContext
    }

    useGenericContext.isProvided = () => {
        return !!use(context)
    }

    return [useGenericContext, context] as const
}

/**
 * Creates context for a component that can inherit properties from
 * the parent tree.
 */
export const createControlledContext = <const TProps, const TRef extends Element>() => {
    const context = createContext<ControlledContext<TProps, TRef>>(undefined)

    const _useSlots = (slot?: string | null) => {
        const ctx = useContext(context)

        if (ctx && "slots" in ctx) {
            const allSlots = Object.keys(ctx.slots)
            const formattedSlots = allSlots.map((slot) => `"${slot}"`).join(", ")

            if (!slot && !ctx.slots[DEFAULT_SLOT]) {
                throw new Error(`A slot prop is required. Valid slot names are ${formattedSlots}.`)
            }

            const slotKey = slot || DEFAULT_SLOT

            if (!ctx.slots[slotKey]) {
                throw new Error(`Invalid slot "${slot}". Valid slot names are ${formattedSlots}.`)
            }

            return ctx.slots[slotKey]
        }

        return ctx
    }

    // , ref: ForwardedRef<TRef>
    function _useContext<T>(props: T & SlotProps & { ref?: RefObject<TRef> }) {
        const { ref: contextRef, ...contextProps } =
            _useSlots(props.slot) ?? ({} as Exclude<ReturnType<typeof _useSlots>, null | undefined>)

        const { ref, ...mergedProps } = mergeProps(contextProps, props) as T &
            TProps & { ref?: RefObject<TRef> }
        const mergedRef = useObjectRef(
            useMemo(() => mergeRefs(props.ref, contextRef), [props.ref, contextRef]),
        )

        return [mergedProps, mergedRef] as const
    }

    _useContext.isProvided = () => {
        return !!useContext(context)
    }

    return [_useContext, context.Provider] as const
}

/**
 * Creates context for a component that can inherit properties from
 * the parent tree.
 */
export const createNewControlledContext = <
    const TProps,
    const TRef extends Element,
>(): IsolatedContext<TProps, TRef> => {
    const context = createContext<ControlledContext<TProps, TRef>>(undefined) as IsolatedContext<
        TProps,
        TRef
    >

    const _useSlots = (slot?: string | null) => {
        const ctx = useContext(context)

        if (ctx && "slots" in ctx) {
            const allSlots = Object.keys(ctx.slots)
            const formattedSlots = allSlots.map((slot) => `"${slot}"`).join(", ")

            if (!slot && !ctx.slots[DEFAULT_SLOT]) {
                throw new Error(`A slot prop is required. Valid slot names are ${formattedSlots}.`)
            }

            const slotKey = slot || DEFAULT_SLOT

            if (!ctx.slots[slotKey]) {
                throw new Error(`Invalid slot "${slot}". Valid slot names are ${formattedSlots}.`)
            }

            return ctx.slots[slotKey]
        }

        return ctx
    }

    // , ref: ForwardedRef<TRef>
    function _useContext(
        props: TProps & SlotProps & { ref?: RefObject<TRef> },
    ): readonly [TProps, RefObject<TRef>] {
        const { ref: contextRef, ...contextProps } =
            _useSlots(props.slot) ?? ({} as Exclude<ReturnType<typeof _useSlots>, null | undefined>)

        const { ref, slot, ...mergedProps } = mergeProps(contextProps, props) as TProps & {
            ref?: RefObject<TRef>
        } & SlotProps
        const mergedRef = useObjectRef(useMemo(() => mergeRefs(ref, contextRef), [ref, contextRef]))

        return [mergedProps as TProps, mergedRef] as const
    }

    context.useContext = _useContext
    context.isProvided = () => {
        return !!useContext(context)
    }

    return context
}

export function createNewGenericContext<const T>(nullable: true): IsolatedGenericContext<T | null>
export function createNewGenericContext<const T>(nullable?: false): IsolatedGenericContext<T>
export function createNewGenericContext<const T>(nullable?: boolean) {
    const context = createContext<T>(undefined as never) as IsolatedGenericContext<T>

    function useGenericContext() {
        const maybeContext = use(context)

        if (!maybeContext && !nullable) {
            throw new Error(`useGenericContext must be used in scope of its provider`)
        }

        return maybeContext
    }

    function useGuaranteedContext() {
        const maybeContext = use(context)

        if (!maybeContext) {
            throw new Error(`useGenericContext must be used in scope of its provider`)
        }

        return maybeContext
    }

    context.isProvided = () => {
        return !!use(context)
    }

    context.use = useGenericContext
    context.useContext = useGenericContext
    context.useGuaranteedContext = useGuaranteedContext as () => Exclude<T, null>

    return context
}

export type IsolatedGenericContext<T> = Context<T> & {
    use: () => T
    useContext: () => T
    useGuaranteedContext: () => Exclude<T, null>
    isProvided: () => boolean
}

export type IsolatedContext<TProps, TRef extends Element> = Context<
    ControlledContext<TProps, TRef>
> & {
    isProvided: () => boolean
    useContext: (
        props: TProps & SlotProps & { ref?: RefObject<TRef> },
    ) => readonly [Omit<TProps, "slot">, RefObject<TRef>]
}
