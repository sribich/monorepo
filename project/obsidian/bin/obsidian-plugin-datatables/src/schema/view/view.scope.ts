import { scope, type } from "arktype"
import type { Compute } from "ts-toolbelt/out/Any/Compute"

import { propertyFilters } from "../property/property"

export const viewScope = type({
    properties: "string[]",
    filters: type(
        scope({
            property: type(
                type({
                    uuid: "string",
                    property: "string",
                }),
                "&",
                propertyFilters,
            ),
            compound: {},
            union: "property | compound",
        }).export().union,
        "[]",
    ),
})

export type ViewScope = (typeof viewScope)["infer"]
export type ViewFilter = (typeof viewScope.infer.filters)[number]

export type WithViewFilterMetadata<T> = T extends never
    ? never
    : ((arg: T) => never) extends (arg: infer I) => void
      ? Compute<I & { uuid: string; property: string }>
      : never
