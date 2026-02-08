// @ts-nocheck
import { props as stylexProps } from "@stylexjs/stylex"
import type { CompiledStyles, InlineStyles, StyleXArray } from "@stylexjs/stylex/lib/StyleXTypes"
import { useMemo } from "react"
import type { StyleSlot } from "../utils/props"

type InferBooleanType<T> = Exclude<T, "true" | "false"> extends never ? boolean : T

export type StyleSlots = Record<string, unknown>

export type StyleVariants<TSlots extends StyleSlots> = {
    [variant: string]: {
        [variantValues: string]: {
            [K in keyof TSlots]?: unknown
        }
    }
}

export type StyleModifiers<TSlots extends StyleSlots> = {
    [modifier: string]: {
        [K in keyof TSlots]?: unknown
    }
}

export type StyleConditions<TSlots extends StyleSlots> = {
    [condition: string]:
        | {
              true?: Partial<Record<keyof TSlots, unknown>>
              false?: Partial<Record<keyof TSlots, unknown>>
          }
        | Partial<Record<keyof TSlots, unknown>>
}

export type StyleCompounds<
    TSlots extends StyleSlots,
    TVariants extends StyleVariants<TSlots>,
    TConditions extends StyleConditions<TSlots>,
> = Array<{
    variants?: {
        [K in keyof TVariants]?: keyof TVariants[K] | Array<keyof TVariants[K]>
    }
    conditions?: {
        [K in keyof TConditions]?: boolean
    }
    modify?: {
        [K in keyof TVariants]?: {
            [VK in keyof TVariants[K]]?: unknown
        }
    }
    apply?: {
        [K in keyof TSlots]?: unknown
    }
}>

export interface RawStyles<
    TSlots extends StyleSlots,
    TVariants extends StyleVariants<TSlots>,
    TModifiers extends StyleModifiers<TSlots>,
    TConditions extends StyleConditions<TSlots>,
    TCompounds extends StyleCompounds<TSlots, TVariants, TConditions>,
> {
    slots: TSlots
    variants?: TVariants

    defaultVariants?: {
        [K in keyof TVariants]: InferBooleanType<keyof TVariants[K]> // extends "true" | "false" ? boolean : keyof TVariants[K]
    }

    modifiers?: TModifiers
    conditions?: TConditions
    compounds?: TCompounds
}

export type InferRawStyles<T> =
    T extends RawStyles<
        infer _TSlots,
        infer _TVariants,
        infer _TModifiers,
        infer _TConditions,
        infer _TCompounds
    >
        ? T
        : `$Value is not a valid style definition`

///=============================================================================
/// MakeStyles
///=============================================================================
/**
 * The styleId is used for caching the instantiated style internals. See
 * `useStyles` for the caching.
 */
let styleId = 0

export type MappedCondition<T extends object> = keyof T extends "true" | "false" ? T : { true: T }

export type MappedConditions<T> =
    T extends InferRawStyles<infer R>
        ? Omit<T, "conditions"> & {
              conditions: { [K in keyof R["conditions"]]: MappedCondition<R["conditions"][K]> }
          }
        : never

export type MadeStyles<TStyles> = MappedConditions<InferRawStyles<TStyles>> & {
    $$variantKeys: string[]
    $$conditionKeys: string[]
    $$styleId: number
}

export type ExportedStyles = MadeStyles<{ slots: StyleSlots }>

export const makeStyles = <const TStyles>(style: InferRawStyles<TStyles>): MadeStyles<TStyles> => {
    if (typeof style === "string") {
        throw new Error("invalid styles")
    }

    const madeStyles = style as MadeStyles<TStyles>

    madeStyles.$$variantKeys = Object.keys(style.variants ?? {})
    madeStyles.$$conditionKeys = Object.keys(style.conditions ?? {})

    madeStyles.$$styleId = styleId++

    return madeStyles
}

///=============================================================================
/// VariantProps
///=============================================================================
export type VariantProps<T> = T extends `$${string}`
    ? T
    : T extends StyleLike<any, any, any, any, any>
      ? {
            [K in keyof T["variants"]]?: keyof T["variants"][K] extends "true" | "false"
                ? boolean
                : keyof T["variants"][K]
        } & {
            [K in keyof T["conditions"]]?: keyof T["conditions"][K] extends "true" | "false"
                ? boolean
                : keyof T["conditions"][K]
        }
      : `$OtherError`

/**
 *
 */
export type InferStyles<T> =
    T extends StyleLike<
        infer _TSlots,
        infer _TVariants,
        infer _TModifiers,
        infer _TConditions,
        infer _TCompounds
    >
        ? T
        : `$Value is not a valid stylex component.`

export type CachedStyle<
    Conditions extends Record<string, Record<string, any>>,
    TStyle extends string,
> = ((...additionalStyles: unknown[]) => { className: string }) & {
    [K in keyof Conditions]: Conditions[K][TStyle]
}

export type CachedStyles<TStyles> = TStyles extends string
    ? never
    : TStyles extends StyleLike<
            infer _,
            infer TVariants,
            infer TModifiers,
            infer TConditions,
            infer TCompounds
        >
      ? {
            styles: {
                [P in Extract<keyof TStyles["slots"], string>]: CachedStyle<
                    TConditions & TModifiers,
                    P
                >
            }
            values: Required<VariantProps<TStyles>>
        }
      : never

export interface StyleLike<
    TSlots extends StyleSlots,
    TVariants extends StyleVariants<TSlots>,
    TModifiers extends StyleModifiers<TSlots>,
    TConditions extends StyleConditions<TSlots>,
    TCompounds extends StyleCompounds<TSlots, TVariants, TConditions>,
> {
    slots: TSlots
    variants: TVariants

    defaultVariants: {
        [K in keyof TVariants]: keyof TVariants[K] | boolean
    }

    modifiers?: TModifiers
    conditions?: TConditions
    compounds?: TCompounds

    /**
     * @private
     */
    $$variantKeys: string[]

    /**
     * @private
     */
    $$conditionKeys: string[]

    /**
     * @private
     */
    $$styleId: number
}

///
const styleCache: Record<number, any> = {}

export const useStyles = <const TStyles, const TInferredStyles extends InferStyles<TStyles>>(
    style: InferStyles<TStyles>,
    props?: VariantProps<TInferredStyles>,
): CachedStyles<TInferredStyles> => {
    if (typeof style === "string") {
        throw new Error("invalid styles")
    }

    if (!styleCache[style.$$styleId]) {
        styleCache[style.$$styleId] = {}
    }

    const cachedStyles = styleCache[style.$$styleId]

    // const dependencies = [
    //     ...style.$$variantKeys.map((key) => props?.[key as never]),
    //     ...style.$$conditionKeys.map((key) => props?.[key as never]),
    // ]

    return useMemo(
        () => {
            const propValues = Object.fromEntries(
                [...style.$$variantKeys, ...style.$$conditionKeys].map((variant) => [
                    variant,
                    props?.[variant as never] ?? style.defaultVariants[variant as never],
                ]),
            )

            const styles = Object.fromEntries(
                Object.entries(style.slots).map(([slot, slotValue]) => {
                    const getCompounds = () => {
                        const result = []

                        if (!style.compounds) {
                            return []
                        }

                        comp: for (const compound of style.compounds) {
                            if (compound.variants) {
                                for (const variant in compound.variants) {
                                    let value = compound.variants[variant]

                                    if (typeof value === "boolean") {
                                        value = String(value)
                                    }

                                    if (
                                        Array.isArray(value) &&
                                        value.includes(propValues[variant])
                                    ) {
                                        // Value exists, check next variant
                                        continue
                                    }

                                    if (value !== propValues[variant]) {
                                        continue comp
                                    }
                                }
                            }

                            if (compound.conditions) {
                                for (const condition in compound.conditions) {
                                    let value = compound.conditions[condition]

                                    if (typeof value === "boolean") {
                                        value = String(value) as "true" | "false"
                                    }

                                    if (value !== String(propValues[condition])) {
                                        continue comp
                                    }
                                }
                            }

                            if (compound.apply) {
                                const comp = compound.apply?.[slot]

                                if (comp) {
                                    result.push(comp)
                                }
                            }

                            if (compound.modify) {
                                for (const modifier in compound.modify) {
                                    let value = propValues[modifier]

                                    if (typeof value === "boolean") {
                                        value = String(value)
                                    }

                                    const comp = compound.modify?.[modifier]?.[value]?.[slot]

                                    if (comp) {
                                        result.push(comp)
                                    }
                                }
                            }
                        }

                        return result
                    }

                    function makeStyle(
                        ...additionalProps: readonly StyleXArray<
                            | (null | undefined | CompiledStyles)
                            | boolean
                            | Readonly<[CompiledStyles, InlineStyles]>
                        >[]
                    ) {
                        if (typeof style === "string") {
                            return
                        }

                        const result = [
                            slotValue,
                            additionalProps,
                            style.variants &&
                                Object.entries(style.variants).map(([variant, variantSlots]) => {
                                    let variantValue = propValues[variant]

                                    if (typeof variantValue === "boolean") {
                                        // @ts-ignore
                                        variantValue = variantValue.toString()
                                    }

                                    if (variantValue && variantValue in variantSlots) {
                                        return variantSlots[variantValue][slot]
                                    }
                                }),
                            style.modifiers &&
                                Object.entries(style.modifiers).map(([variant, variantSlots]) => {
                                    let variantValue = propValues[variant]

                                    if (typeof variantValue === "boolean") {
                                        // @ts-ignore
                                        variantValue = variantValue.toString()
                                    }

                                    if (variantValue && variantValue in variantSlots) {
                                        return variantSlots[variantValue][slot]
                                    }
                                }),
                            style.conditions &&
                                Object.entries(style.conditions).map(([variant, variantSlots]) => {
                                    let variantValue = propValues[variant]

                                    if (!variantValue) {
                                        variantValue = "false"
                                    }
                                    if (typeof variantValue === "boolean") {
                                        // @ts-ignore
                                        variantValue = variantValue.toString()
                                    }

                                    if (variantValue && variantValue in variantSlots) {
                                        return variantSlots[variantValue][slot]
                                    }

                                    if (variantValue === "true" && slot in variantSlots) {
                                        return variantSlots[slot]
                                    }
                                }),
                            getCompounds(),

                            props?.stylexProps,
                        ]

                        return stylexProps(result)
                    }

                    makeStyle.propless = (
                        ...additionalProps: readonly StyleXArray<
                            | (null | undefined | CompiledStyles)
                            | boolean
                            | Readonly<[CompiledStyles, InlineStyles]>
                        >[]
                    ) => {
                        if (typeof style === "string") {
                            return
                        }

                        return [
                            slotValue,
                            Object.entries(style.variants).map(([variant, variantSlots]) => {
                                let variantValue = propValues[variant]

                                if (typeof variantValue === "boolean") {
                                    // @ts-ignore
                                    variantValue = variantValue.toString()
                                }

                                if (variantValue && variantValue in variantSlots) {
                                    return variantSlots[variantValue][slot]
                                }
                            }),
                            getCompounds(),
                            additionalProps,
                            props?.stylexProps,
                        ]
                    }

                    Object.entries(style.conditions ?? {}).forEach(([condition, value]) => {
                        if (slot in value) {
                            // @ts-expect-error TODO figure out how to fix this
                            makeStyle[condition] = value[slot as never]
                        }
                    })

                    Object.entries(style.modifiers ?? {}).forEach(([condition, value]) => {
                        if (slot in value) {
                            // @ts-expect-error TODO figure out how to fix this
                            makeStyle[condition] = value[slot as never]
                        }
                    })

                    return [slot, makeStyle]
                }),
            ) as MadeStyles<TStyles>

            return { styles, values: propValues } as never as CachedStyles<TInferredStyles>
        },
        Object.values(props ?? {}),
    )
}

/**
 *
 */

/*
export const makeStyles = <
    const TStyles extends RawStyles<A, B, C>,
    const A extends Record<string, unknown>,
    const B extends Record<string, Record<keyof A, unknown>>,
    const C extends Record<string, Record<keyof A, unknown>>,
>(
    style: TStyles,
): TStyles & { $$variantKeys: string[] } => {
    if (typeof style === "string") {
        throw new Error(`TODO`)
    }

    return {
        ...style,
        $$variantKeys: Object.keys(style.variants)
    }
}

*/

const anyStyles = makeStyles({
    slots: {},
    // conditions: {},
    variants: {},
    defaultVariants: {},
})

export const stylesToArgTypes = (styles: typeof anyStyles) => {
    return [...Object.entries(styles.variants), ...Object.entries(styles.conditions ?? {})]
        .map(([variantName, variantData]) => {
            const keys = Object.keys(variantData as object)

            if (keys.length === 0) {
                return
            }

            // A condition will have either a true/false key, or it will be a modifier.
            const isCondition =
                keys.every((key) => ["true", "false"].includes(key)) ||
                "$$css" in variantData[keys[0]]

            const control = isCondition ? "boolean" : "select"
            const options = isCondition ? undefined : keys

            return [
                variantName,
                {
                    args: styles.defaultVariants[variantName],
                    argTypes: {
                        control: { type: control },
                        options,
                        table: {
                            defaultValue: { summary: styles.defaultVariants[variantName] },
                        },
                    },
                },
            ] as const
        })
        .filter(Boolean)
        .reduce(
            (prev, curr) => {
                prev.args[curr[0]] = curr[1].args
                prev.argTypes[curr[0]] = curr[1].argTypes

                return prev
            },
            {
                args: {},
                argTypes: {},
            },
        )
}
