import type { Compute } from "ts-toolbelt/out/Any/Compute"

type ScalarType =
    | string
    | number
    | bigint
    | boolean
    | null
    | undefined
    | unknown[]
    | Map<unknown, unknown>
    | Set<unknown>
    | Date
    | RegExp
    | AbortController
    | ((...args: unknown[]) => unknown)

type IsAny<T> = unknown extends T ? ([keyof T] extends [never] ? false : true) : false

/**
 * Converts a complex object representation into a union of
 * dot notation accesses.
 *
 * @example
 *
 * ```typscript
 * type DotKeys = RecursiveKeyOf<{a: {b: {c: string}}> // "a" | "a.b" | "a.b.c"
 * ```
 */
export type RecursiveKeyOf<T, Prefix extends string = never> = T extends ScalarType
    ? never
    : IsAny<T> extends true
      ? never
      : {
            [K in keyof T & string]: [Prefix] extends [never]
                ? K | RecursiveKeyOf<T[K], K>
                : `${Prefix}.${K}` | RecursiveKeyOf<T[K], `${Prefix}.${K}`>
        }[keyof T & string]

/**
 * Get the type of a nested property with dot syntax
 *
 * Basically the inverse of `RecursiveKeyOf`
 *
 * @example
 * type t = DeepPropertyType<{a: {b: {c: string}}}, 'a.b.c'> // => string
 */
export type DeepPropertyType<
    TObj,
    TProperty extends RecursiveKeyOf<TObj>,
> = TProperty extends `${infer Prefix}.${infer Rest}`
    ? Prefix extends keyof TObj
        ? Rest extends RecursiveKeyOf<TObj[Prefix]>
            ? DeepPropertyType<TObj[Prefix], Rest>
            : never
        : never
    : TProperty extends keyof TObj
      ? TObj[TProperty]
      : never

const isValidPath = <K extends string>(key: K, obj: unknown): obj is Record<K, unknown> => {
    return !!obj && typeof obj === "object" && key in obj
}

export const getValueFromPath = <
    const TObj extends Record<string, unknown>,
    TPath extends RecursiveKeyOf<TObj> & string,
>(
    obj: TObj,
    path: TPath,
): DeepPropertyType<TObj, TPath> => {
    const segments = path.split(".")

    let item: unknown = obj

    for (const path of segments) {
        if (isValidPath(path, item)) {
            item = item[path]
            continue
        }

        throw new Error("TODO: Make into Error")
    }

    return item as DeepPropertyType<TObj, TPath>
}

export const setValueFromPath = <
    const TObj extends Record<string, unknown>,
    TPath extends RecursiveKeyOf<TObj> & string,
    TValue extends DeepPropertyType<TObj, TPath>,
>(
    obj: TObj,
    path: TPath,
    value: TValue,
) => {
    const segments = path.split(".")
    const lastSegment = segments.at(-1)

    const lastNode = segments.slice(0, -1).reduce(
        (acc, curr) => {
            acc[curr] ??= {}
            return acc[curr] as Record<string, unknown>
        },
        obj as Record<string, unknown>,
    )

    if (!lastSegment) {
        // TODO: LogicException? AssertionError? Should never hit this.
        throw new Error("Error")
    }

    lastNode[lastSegment] = value
}

type ExtractedArray<
    TArray extends readonly unknown[],
    TKey extends keyof TArray[number],
    TPath,
> = TArray extends readonly [infer A, ...infer B]
    ? A[TKey] extends string
        ? TPath extends RecursiveKeyOf<A>
            ? B extends []
                ? { [P in A[TKey]]: DeepPropertyType<A, TPath> }
                : { [P in A[TKey]]: DeepPropertyType<A, TPath> } & ExtractedArray<B, TKey, TPath>
            : never
        : never
    : never

type JoinIntersection<T extends Record<string, unknown>> = {
    [K in keyof T]: T[K]
}

export const extract = <
    const TArr extends readonly TItem[],
    TItem extends Record<string, any>,
    TKey extends keyof TArr[number],
    TPath extends RecursiveKeyOf<TArr[number]> & string,
>(
    array: TArr,
    key: TKey,
    path: TPath,
): JoinIntersection<ExtractedArray<TArr, TKey, TPath>> => {
    return Object.fromEntries(array.map((it) => [it[key], getValueFromPath(it, path)]))
}
