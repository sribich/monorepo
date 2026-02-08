import { mergeIds } from "@react-aria/utils"
import type { TupleToUnion, UnionToIntersection } from "@sribich/ts-utils"
import { chain } from "react-aria"

import { clsx } from "./clsx"

/**
 * Merges multiple individual prop objects together.
 *
 *  - Event handlers that begin with 'on' are chained together.
 *  - classNames are joined together
 *  - styles are merged
 *  - ids are merged
 *
 * Other props are overridden in a last prop wins manner.
 */
export const mergeProps = <T extends (Record<string, any> | null | undefined)[]>(
    ...args: T
): UnionToIntersection<TupleToUnion<T>> => {
    const mergedProps = { ...args[0] }

    for (let i = 1; i < args.length; i++) {
        const props = args[i]

        for (const prop in props) {
            const prev = mergedProps[prop]
            const next = props[prop]

            if (
                typeof prev === "function" &&
                typeof next === "function" &&
                prop[0] === "o" &&
                prop[1] === "n" &&
                prop.charCodeAt(2) >= 65 /* 'A' */ &&
                prop.charCodeAt(2) <= 90 /* 'Z' */
            ) {
                mergedProps[prop] = chain(prev, next)
            } else if (
                prop === "className" &&
                typeof prev === "string" &&
                typeof next === "string"
            ) {
                // The use of clsx may be unintuitive here, but it exists in order
                // to ensure things such as snapshot tests behave properly in case
                // a type swaps between "" and null, for example. A simple string
                // concatenation would not suffice here. (And for memoization)
                mergedProps[prop] = clsx(prev, next)
            } else if (prop === "style" && typeof prev === "object" && typeof next === "object") {
                mergedProps[prop] = { ...prev, ...next }
            } else if (prop === "id" && typeof prev === "string" && typeof next === "string") {
                mergedProps[prop] = mergeIds(prev, next)
            } else {
                mergedProps[prop] = next !== undefined ? next : prev
            }
        }
    }

    return mergedProps as UnionToIntersection<TupleToUnion<T>>
}
