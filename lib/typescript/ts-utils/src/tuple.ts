export type TupleToUnion<T> = { [P in keyof T]: T[P] } extends { [key: number]: infer V }
    ? V extends null | undefined
        ? Record<string, never>
        : V
    : never

/**
 * Distribute a union to a contravariant position.
 *
 * @see https://www.typescriptlang.org/docs/handbook/release-notes/typescript-2-8.html#distributive-conditional-types
 * @see https://www.typescriptlang.org/docs/handbook/release-notes/typescript-2-8.html#type-inference-in-conditional-types
 */
export type UnionToIntersection<TUnion> = (
    TUnion extends never
        ? never
        : (arg: TUnion) => never
) extends (arg: infer I) => void
    ? I
    : never

export type UnionToTuple<T> =
    UnionToIntersection<T extends never ? never : (t: T) => T> extends (_: never) => infer U
        ? Exclude<T, U> extends never
            ? [T]
            : [...UnionToTuple<Exclude<T, U>>, U]
        : never

export type JoinUnion<T, TDelim extends string> = JoinTuple<UnionToTuple<T>, TDelim>

export type JoinTuple<TItems extends unknown[], TDelim extends string> = TItems extends readonly [
    infer A,
    ...infer B,
]
    ? A extends string
        ? B extends []
            ? `${A}`
            : `${A}${TDelim}${JoinTuple<B, TDelim>}`
        : never
    : never
