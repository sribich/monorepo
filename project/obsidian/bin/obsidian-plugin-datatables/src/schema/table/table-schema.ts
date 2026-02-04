import { type } from "arktype"

import { propertySchema } from "../property/property-schema"
import { templateSchema } from "../template/template-schema"
import { viewSchema } from "../view/view-schema"

export const tableSchema = type({
    properties: type(propertySchema, "[]"),
    views: type(viewSchema, "[]"),
    templates: templateSchema,
})

export type TableSchema = (typeof tableSchema)["infer"]
