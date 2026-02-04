import type { JoinUnion } from "@sribich/ts-utils"
import { scope } from "arktype"

// import { JoinUnion } from "../../../../types/tuple"
import { extract } from "../../ui/util/array"
import { backreference } from "./backreference/backreference"
import { checkbox } from "./checkbox/checkbox"
import { date } from "./date/date"
import { number } from "./number/number"
import type { PropertyKind } from "./property-kind"
import { reference } from "./reference/reference"
import { select } from "./select/select"
import { text } from "./text/text"
import { title } from "./title/title"

export const properties = [
    text,
    number,
    select,
    date,
    checkbox,
    reference,
    backreference,
    title,
] as const

export const property = scope({
    ...extract(properties, "kind", "config.schema"),
    union: Object.values(properties)
        .map((it) => it.kind)
        .join(" | ") as JoinUnion<(typeof properties)[number]["kind"], " | ">,
}).export().union

export const field = scope({
    ...extract(properties, "kind", "field.schema"),
    union: Object.values(properties)
        .map((it) => it.kind)
        .join(" | ") as JoinUnion<(typeof properties)[number]["kind"], " | ">,
}).export().union

const possiblePropertyFilters = properties.filter((it) => Object.keys(it.filter.filters).length > 0)

export const propertyFilters = scope({
    ...extract(possiblePropertyFilters, "kind", "filter.type"),
    union: Object.values(possiblePropertyFilters)
        .map((it) => it.kind)
        .join(" | ") as JoinUnion<(typeof properties)[number]["kind"], " | ">,
}).export().union

export type PropertyRepr<TKind extends PropertyKind> = Extract<
    (typeof properties)[number],
    { kind: TKind }
>

export type PropertySchemaRepr<TKind extends PropertyKind> = Extract<
    typeof property.infer,
    { kind: TKind }
>

export type PropertyConfigRepr<TKind extends PropertyKind> = Extract<
    typeof property.infer,
    { kind: TKind }
>["config"]

export type PropertyFieldRepr<TKind extends PropertyKind> = Extract<
    typeof field.infer,
    { kind: TKind }
>["value"]

export const getProperty = <const TKind extends PropertyKind>(kind: TKind): PropertyRepr<TKind> => {
    const definition = properties.find((it) => it.kind === kind)

    if (!definition) {
        throw new Error(`Attempted to get field metadata for unknown kind: ${kind}`)
    }

    return definition as PropertyRepr<TKind>
}
