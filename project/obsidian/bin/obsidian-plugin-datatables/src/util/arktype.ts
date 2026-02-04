import { ArkErrors, type Type, type } from "arktype"

export const validateOrThrow = <T extends Type<unknown>>(type: T, rawData: unknown): T["infer"] => {
    const data = type(rawData)

    if (data instanceof ArkErrors) {
        throw new Error(`Invalid data: ${data.summary}`)
    }

    return data
}

/**
 * TODO: Some shit might happen here where value gets fucked.
 */
export const enumVariant = <TVariant extends string>(variant: TVariant) => {
    return type("string").narrow((value: unknown): value is TVariant => {
        return value === variant
    })
}

export const parseDate = type("string").pipe((value): Date => new Date(value))

/**
 * TODO: Some shit might happen here where value gets fucked.
 */
export const dateFilterType = type("'today' | 'tomorrow' | Date").pipe((value): Date => {
    if (value instanceof Date) {
        return value
    }

    return new Date(value)

    // const date = new Date(value)
    // return Number.isNaN(date.getTime()) ? value : date
})

export const EnumType = <
    EnumObject extends Record<string, string>,
    EnumType extends EnumObject[keyof EnumObject] = EnumObject[keyof EnumObject],
>(
    Enum: EnumObject,
) => {
    const values = Object.values(Enum)

    return type("string").narrow((value: string): value is EnumType => {
        return values.includes(value)
    })
}
