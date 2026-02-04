import { type Infer, type } from "arktype"

import { makeView } from "../view-definition"

export const boardView = makeView("board")({
    name: "Board",
    layout: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
    },
    config: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
    },
})
