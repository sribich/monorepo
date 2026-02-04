import { type Infer, type } from "arktype"

import { makeView } from "../view-definition"

export const timelineView = makeView("timeline")({
    name: "Timeline",
    layout: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
    },
    config: {
        default: {},
        type: type({} as Infer<Record<string, never>>),
    },
})
