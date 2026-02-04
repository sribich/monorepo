import { type } from "arktype"

import { makeProperty } from "../property-definition"

export const backreference = makeProperty("backreference")({
    name: "Backreference",
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
        type: "string[]",
        morphs: {},
    },
    filter: {
        type: type("never"),
        filters: {},
    },
})
