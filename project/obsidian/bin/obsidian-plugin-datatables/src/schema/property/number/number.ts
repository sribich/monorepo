import { type Infer, type } from "arktype"

import { makeProperty } from "../property-definition"

export const number = makeProperty("numbers")({
    name: "Number",
    config: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
        morphs: {},
    },
    field: {
        default: 0,
        type: type("number"),
        morphs: {},
    },
    filter: {
        type: type("never"),
        filters: {},
    },
})
