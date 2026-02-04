import { type MarkdownPostProcessorContext, Plugin } from "obsidian"

import { Index } from "../index/index"
import type { SchemaLoader } from "../schema/schema-loader"
import { datatableProcessor } from "./processors/datatable"
import { datatableSchemaProcessor } from "./processors/datatable-schema"
import { RenderHost, Renderer } from "./render"

export interface HostContext {
    plugin: Plugin
    index: Index
    schema: SchemaLoader
}

export interface ProcessorContext {
    source: string
    element: HTMLElement
    context: MarkdownPostProcessorContext
}

export interface Processor {
    language: string
    createRenderer: () => Renderer
}

const processors = [datatableProcessor, datatableSchemaProcessor]

export const loadProcessors = (host: HostContext) => {
    for (const processor of processors) {
        host.plugin.registerMarkdownCodeBlockProcessor(
            processor.language,
            (source, element, context) => {
                const processorContext = { source, element, context }
                const child = new RenderHost(processor, host, processorContext)

                context.addChild(child)

                const reloadComponent = () => {
                    document["__dev__dt_cleanup_hooks"] ??= []
                    document["__dev__dt_cleanup_hooks"].push(() => {
                        context.removeChild(child)
                        // context.forceReload()

                        setTimeout(() => {
                            context.forceReload()
                        }, 250)

                        reloadComponent()
                    })
                }

                reloadComponent()
            },
        )
    }
}
