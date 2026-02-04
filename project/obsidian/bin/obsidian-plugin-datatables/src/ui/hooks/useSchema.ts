import { createGenericContext } from "@sribich/fude"

import { Schema } from "../../schema/schema"

export const [useSchema, SchemaProvider] = createGenericContext<Schema>()
