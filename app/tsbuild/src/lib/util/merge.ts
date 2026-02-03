import assert from "assert"

interface Object {
    [key: string]: unknown
}

type TAllKeys<T> = T extends any ? keyof T : never

type TIndexValue<T, K extends PropertyKey, D = never> = T extends any
    ? K extends keyof T
        ? T[K]
        : D
    : never

type TPartialKeys<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>> extends infer O
    ? { [P in keyof O]: O[P] }
    : never

type TFunction = (...a: any[]) => any

type TPrimitives = string | number | boolean | bigint | symbol | Date | TFunction

type TMerged<T> = [T] extends [Array<any>]
    ? { [K in keyof T]: TMerged<T[K]> }
    : [T] extends [TPrimitives]
      ? T
      : [T] extends [object]
        ? TPartialKeys<{ [K in TAllKeys<T>]: TMerged<TIndexValue<T, K>> }, never>
        : T

const isObject = (obj: unknown) => {
    if (typeof obj === "object" && obj !== null) {
        if (typeof Object.getPrototypeOf === "function") {
            const prototype = Object.getPrototypeOf(obj)
            return prototype === Object.prototype || prototype === null
        }

        return Object.prototype.toString.call(obj) === "[object Object]"
    }

    return false
}

export const merge = <A, B>(from: A, into: B): TMerged<A & B> =>
    [from, into].reduce(
        (result, current) => {
            if (Array.isArray(current)) {
                throw new TypeError("Arguments must be objects, not arrays")
            }

            assert(typeof current === "object")

            for (const key in current) {
                // @ts-ignore
                if (Array.isArray(result[key]) && Array.isArray(current[key])) {
                    // @ts-ignore
                    result[key] = current[key]
                    // @ts-ignore
                } else if (isObject(result[key]) && isObject(current[key])) {
                    // @ts-ignore
                    result[key] = merge(result[key] as Object, current[key] as Object)
                } else {
                    // @ts-ignore
                    result[key] = current[key]
                }
            }

            return result
        },
        {} as TMerged<A & B>,
    ) as TMerged<A & B>
