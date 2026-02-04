import type { Immutable } from "@sribich/ts-utils"

import type { PropertySchemaRepr } from "../../schema/property/property"
import type { PropertyKind } from "../../schema/property/property-kind"
import type { PropertySchema } from "../../schema/property/property-schema"

export const isPropertyKind = <TKind extends PropertyKind>(
    property: Immutable<PropertySchema>,
    kind: TKind,
): property is PropertySchemaRepr<`${TKind}`> => {
    return property.kind === kind
}

export const assertProperty = <TKind extends PropertyKind>(
    property: Immutable<PropertySchema>,
    kind: TKind,
): PropertySchemaRepr<`${TKind}`> => {
    if (!isPropertyKind(property, kind)) {
        throw new Error(`Invalid kind passed to component. Expected ${kind}. Got ${property.kind}`)
    }

    return property
}
