import { type } from "arktype"

import { makeProperty } from "../property-definition"

export const reference = makeProperty("reference")({
    name: "Reference",
    config: {
        default: {
            target: "",
        },
        type: type({
            target: "string",
        }),
        morphs: {},
    },
    field: {
        default: [],
        type: type("string[]"),
        morphs: {},
    },
    filter: {
        type: type("never"),
        filters: {},
    },
})
