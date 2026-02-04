import { type } from "arktype"

export const templateSchema = type({
    "default?": "string | null",
    options: type(
        {
            uuid: "string",
            name: "string",
            path: "string",
        },
        "[]",
    ),
})

export type TemplateSchema = (typeof templateSchema)["infer"]
