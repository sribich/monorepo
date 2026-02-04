import { mergeIds } from "@react-aria/utils"
import type { TupleToUnion, UnionToIntersection } from "@sribich/ts-utils"
import { clsx } from "clsx"
import { chain } from "react-aria"

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
    // Start with the first prop object already "merged" since there
    // is no processing to be done in this case.
    const mergedProps = { ...args[0] }

    for (let i = 1; i < args.length; i++) {
        const props = args[i]

        // TODO(perf): This might be faster if we use either, need to benchmark.
        //               for (const prop of Object.values(props))
        //             or
        //               for (var i = 0, keys = Object.keys(object); i < keys.length; i++)
        for (const prop in props) {
            const prev = mergedProps[prop]
            const next = props[prop]

            if (
                typeof prev === "function" &&
                typeof next === "function" &&
                prop[0] === "o" &&
                prop[1] === "n" &&
                prop.charAt(2) === prop.charAt(2).toUpperCase()
            ) {
                mergedProps[prop] = chain(prev, next)
            } else if (
                prop === "className" &&
                typeof prev === "string" &&
                typeof next === "string"
            ) {
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
