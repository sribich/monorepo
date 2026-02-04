import type { Immutable } from "@sribich/ts-utils"
import { proxy } from "valtio"

import type { Document } from "../../index/document"
import { SchemaLoader } from "../../schema/schema-loader"
import { DatatableSchemaRoot } from "../../ui/features/datatable-schema/DatatableSchemaRoot"
import { type Processor } from "../processor"
import { ReactRenderer } from "../renderers/react"

export interface DatatableSchemaContext {
    proxy: {
        schemaRevision: number
        indexRevision: number
        document: Immutable<Document>
    }
    loader: SchemaLoader
}

export const datatableSchemaProcessor = {
    language: "datatable-schema",
    createRenderer: () => {
        return new ReactRenderer({
            mount: {
                component: DatatableSchemaRoot,
                props: {},
            },
            context: async (renderHost) => {
                const path = renderHost.processor.context.sourcePath
                const document = renderHost.host.index.documents.get(path)

                if (!document) {
                    throw new Error(`Unable to render component`)
                }

                const proxyData = proxy({
                    schemaRevision: 0,
                    indexRevision: 0,
                    document,
                })

                renderHost.registerEvent(
                    app.workspace.on("datatables:index:changed", () => {
                        proxyData.indexRevision += 1

                        const newDocument = renderHost.host.index.documents.get(path)
                        if (newDocument) {
                            proxyData.document = newDocument
                        }
                    }),
                )

                renderHost.registerEvent(
                    app.workspace.on("datatables:schema:changed", () => {
                        proxyData.schemaRevision += 1
                    }),
                )

                return {
                    proxy: proxyData,
                    loader: renderHost.host.schema,
                } satisfies DatatableSchemaContext
            },
        })
    },
} satisfies Processor
