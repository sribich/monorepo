import type { ImmutablePrimitive } from "./immutable.js"

export type NonEmptyArray<T> = [T, ...T[]]

export const UNSET_TYPE_CHECK: unique symbol = Symbol.for("tsbuild")

export type DeepRequire<T> = T extends ImmutablePrimitive
    ? T
    : T extends Array<infer U>
      ? ReadonlyArray<Require<U>>
      : T extends Map<infer K, infer V>
        ? ReadonlyMap<Require<K>, Require<V>>
        : T extends Set<infer M>
          ? ReadonlySet<Require<M>>
          : T extends { [K in keyof T]: T[K] }
            ? { readonly [K in keyof T]-?: DeepRequire<T[K]> | typeof UNSET_TYPE_CHECK }
            : {
                  err: `Unable to coerce type to be immutable. The 'errType' type could not be matched.`
                  errType: T
              }

export type Require<T> = { [K in keyof T]-?: T[K] | typeof UNSET_TYPE_CHECK }

export type TypeCheckObjectFilter<T extends object> = {
    -readonly [K in keyof T as T[K] extends typeof UNSET_TYPE_CHECK ? never : K]: Exclude<
        T[K],
        typeof UNSET_TYPE_CHECK
    >
}

export const removeTypeCheckedFields = <T extends object>(obj: T) => {
    return Object.fromEntries(
        Object.entries(obj).filter((entry) => entry[1] !== UNSET_TYPE_CHECK),
    ) as TypeCheckObjectFilter<T>
}
