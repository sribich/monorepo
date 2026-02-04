import { scope } from "arktype"

import { tableSchema } from "./table/table-schema"

/**
 * The schema is the core of the datatable module.
 *
 * It is a simple collection of versioned tables, which we are able
 * to run migrations over.
 *
 * The schema is persisted as a JSON file on the filesystem.
 */
export const schema = scope({
    tableSchema,
    schema: {
        version: "number",
        tables: "Record<string, tableSchema>",
    },
}).export().schema

export const defaultSchema = {
    version: 1,
    tables: {},
} satisfies DtSchema

// TODO: Rename DtSchema to just Schema. Requires renaming the current Schema class
//       probably to something like SchemaWriter or SchemaManager idk.
export type DtSchema = typeof schema.infer
