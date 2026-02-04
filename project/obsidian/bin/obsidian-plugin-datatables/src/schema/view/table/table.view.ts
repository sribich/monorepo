import { type Type, type } from "arktype"

import { makeView } from "../view-definition"

export const tableView = makeView("table")({
    name: "Table",
    layout: {
        default: {},
        type: type({} as Type<Record<string, never>>),
    },
    config: {
        default: {
            properties: [],
        },
        type: type({
            properties: type(
                {
                    id: "string",
                    "width?": "number",
                },
                "[]",
            ),
        }),
    },
})
