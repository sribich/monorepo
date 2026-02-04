import { type Infer, type } from "arktype"

import { makeProperty } from "../property-definition"

/**
 * Title is a special implicit field that should never be manually instantiated.
 *
 * It's used to display the referenced page in datatables.
 */
export const title = makeProperty("title")({
    name: "Title",
    config: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
        morphs: {},
    },
    field: {
        default: "",
        type: type("string"),
        morphs: {},
    },
    filter: {
        type: type("never"),
        filters: {},
    },
})
