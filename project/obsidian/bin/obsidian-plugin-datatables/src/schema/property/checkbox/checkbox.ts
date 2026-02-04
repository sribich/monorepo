import { type Infer, scope, type } from "arktype"

import type { WithViewFilterMetadata } from "../../view/view.scope"
import { makeProperty } from "../property-definition"

export const { checkboxFilters } = scope({
    is: {
        kind: "'IS'",
        data: "boolean",
    },
    is_not: {
        kind: "'IS_NOT'",
        data: "boolean",
    },
    checkboxFilters: "is | is_not",
}).export()

export type CheckboxFilter = WithViewFilterMetadata<typeof checkboxFilters.infer>

export const checkbox = makeProperty("checkbox")({
    name: "Checkbox",
    config: {
        default: {},
        type: type("object" as Infer<Record<string, never>>, { keys: "strict" }),
        morphs: {},
    },
    field: {
        default: false,
        type: type("boolean"),
        morphs: {},
    },
    filter: {
        type: checkboxFilters,
        default: {
            kind: "IS",
            data: true,
        },
        filters: {
            IS: {
                fn: (_, filter, value) => value === filter.data,
                default: true,
            },
            IS_NOT: {
                fn: (_, filter, value) => value !== filter.data,
                default: true,
            },
        },
    },
})
