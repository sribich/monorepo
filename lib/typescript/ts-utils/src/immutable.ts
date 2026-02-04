type ImmutablePrimitive =
    | bigint
    | boolean
    | never
    | null
    | number
    | string
    | symbol
    | undefined
    /**
     * biome-ignore lint/complexity/noBannedTypes: |
     *    Because we're treating all functions as a primitive with this type,
     *    we explicitly want to allow any and all functions, which `Function`
     *    does.
     */
    | Function

export type Immutable<T> = T extends ImmutablePrimitive
    ? T
    : T extends Array<infer U>
      ? ReadonlyArray<Immutable<U>>
      : T extends Map<infer K, infer V>
        ? ReadonlyMap<Immutable<K>, Immutable<V>>
        : T extends Set<infer M>
          ? ReadonlySet<Immutable<M>>
          : T extends { [K in keyof T]: T[K] }
            ? { readonly [K in keyof T]: Immutable<T[K]> }
            : {
                  err: `Unable to coerce type to be immutable. The 'errType' type could not be matched.`
                  errType: T
              }

export type ForceMutable<T> = T extends ImmutablePrimitive
    ? T
    : T extends ReadonlyArray<Immutable<infer U>>
      ? Array<ForceMutable<U>>
      : T extends ReadonlyMap<Immutable<infer K>, Immutable<infer V>>
        ? Map<ForceMutable<K>, ForceMutable<V>>
        : T extends ReadonlySet<Immutable<infer M>>
          ? Set<ForceMutable<M>>
          : { -readonly [K in keyof T]: ForceMutable<T[K]> }

export type Mutable<T> = {
    -readonly [P in keyof T]: T[P]
}
