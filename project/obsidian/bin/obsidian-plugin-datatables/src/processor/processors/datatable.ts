import { type } from "arktype"
import { proxy, subscribe } from "valtio"

import type { SchemaLoader } from "../../schema/schema-loader"
import { DatatableRoot } from "../../ui/features/datatable-root/DatatableRoot"
import { jsonCodeBlock } from "../code-block/JsonCodeBlock"
import type { Processor } from "../processor"
import { ReactRenderer } from "../renderers/react"

export interface DatatableContext {
    proxy: {
        codeBlock: {
            source?: string
            view?: string
        }
        schemaRevision: number
        indexRevision: number
    }
    loader: SchemaLoader
}

const codeBlockType = type({
    "source?": "string",
    "view?": "string",
})

export const datatableProcessor = {
    language: "datatable",
    createRenderer: () => {
        return new ReactRenderer({
            mount: {
                component: DatatableRoot,
                props: {},
            },
            context: async (renderHost, codeBlock) => {
                const { data, update } = await jsonCodeBlock(codeBlock, codeBlockType)

                const proxyData = proxy({
                    codeBlock: data,
                    schemaRevision: 0,
                    indexRevision: 0,
                })

                renderHost.registerEvent(
                    app.workspace.on("datatables:index:changed", () => {
                        proxyData.indexRevision += 1
                    }),
                )

                renderHost.registerEvent(
                    app.workspace.on("datatables:schema:changed", () => {
                        proxyData.schemaRevision += 1
                    }),
                )

                subscribe(proxyData.codeBlock, () => {
                    update(() => proxyData.codeBlock)
                })

                return {
                    proxy: proxyData,
                    loader: renderHost.host.schema,
                } satisfies DatatableContext
            },
        })
    },
} satisfies Processor
