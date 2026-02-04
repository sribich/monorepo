import { type Infer, scope, type } from "arktype"

import { makeProperty } from "../property-definition"

export const textFilters = scope({
    is: {
        kind: "'IS'",
        data: "string",
    },
    is_not: {
        kind: "'IS_NOT'",
        data: "string",
    },
    contains: {
        kind: "'CONTAINS'",
        data: "string",
    },
    does_not_contain: {
        kind: "'DOES_NOT_CONTAIN'",
        data: "string",
    },
    starts_with: {
        kind: "'STARTS_WITH'",
        data: "string",
    },
    ends_with: {
        kind: "'ENDS_WITH'",
        data: "string",
    },
    is_empty: {
        kind: "'IS_EMPTY'",
        data: "never",
    },
    is_not_empty: {
        kind: "'IS_NOT_EMPTY'",
        data: "never",
    },
    union: "is | is_not | contains | does_not_contain | starts_with | ends_with | is_empty | is_not_empty",
}).export().union

export const text = makeProperty("text")({
    name: "Text",
    config: {
        type: type({} as Infer<Record<string, never>>),
        default: {},
        morphs: {},
    },
    field: {
        type: type("string"),
        default: "",
        morphs: {},
    },
    filter: {
        type: textFilters,
        default: {
            kind: "CONTAINS",
            data: "",
        },
        filters: {
            IS: {
                fn: (_property, filter, value) => value === filter.data,
                default: "",
            },
            IS_NOT: {
                fn: (_property, filter, value) => value !== filter.data,
                default: "",
            },
            CONTAINS: {
                fn: (_property, filter, value) => value.includes(filter.data),
                default: "",
            },
            DOES_NOT_CONTAIN: {
                fn: (_property, filter, value) => !value.includes(filter.data),
                default: "",
            },
            STARTS_WITH: {
                fn: (_property, filter, value) => value.startsWith(filter.data),
                default: "",
            },
            ENDS_WITH: {
                fn: (_property, filter, value) => value.endsWith(filter.data),
                default: "",
            },
            IS_EMPTY: {
                fn: (_property, _filter, value) => value.length === 0,
                default: "" as never,
            },
            IS_NOT_EMPTY: {
                fn: (_property, _filter, value) => value.length > 0,
                default: "" as never,
            },
        },
    },
})
