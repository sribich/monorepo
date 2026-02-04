import { MarkdownRenderChild, TFile } from "obsidian"

import type { HostContext, Processor, ProcessorContext } from "./processor"

let RENDER_ID = 0

export class RenderHost extends MarkdownRenderChild {
    public renderId: number

    public renderer!: Renderer

    constructor(
        public readonly impl: Processor,
        public readonly host: HostContext,
        public readonly processor: ProcessorContext,
    ) {
        super(processor.element)

        this.renderId = RENDER_ID += 1
    }

    override onload(): void {
        this.renderer = this.impl.createRenderer()

        const codeBlock = this.createCodeBlockContext()

        this.renderer.render(this, codeBlock)
    }

    override onunload(): void {
        this.renderer.destroy(this)
    }

    private createCodeBlockContext(): CodeBlockContext {
        return {
            read: () => {
                const { lineStart, lineEnd, text } =
                    this.processor.context.getSectionInfo(this.processor.element) ?? {}

                if (!lineStart || !lineEnd || !text) {
                    return ""
                }

                return text
            },
            readContent: () => {
                const { lineStart, lineEnd, text } =
                    this.processor.context.getSectionInfo(this.processor.element) ?? {}

                if (!lineStart || !lineEnd || !text) {
                    return ""
                }

                return text
                    .split("\n")
                    .slice(lineStart + 1, lineEnd)
                    .join("\n")
            },
            writeContent: async (newContent: string) => {
                const file = this.processor.context.sourcePath

                const maybeFile = app.vault.getAbstractFileByPath(file)

                if (!(maybeFile instanceof TFile)) {
                    throw new Error(`Unable to find file ${file} to update codeblock content`)
                }

                await app.vault.process(maybeFile, (currentContent) => {
                    const { lineStart, lineEnd, text } =
                        this.processor.context.getSectionInfo(this.processor.element) ?? {}

                    if (!lineStart || !lineEnd || !text) {
                        return currentContent
                    }

                    const currentSplit = currentContent.split("\n")
                    const newSplit = newContent.split("\n")

                    return [
                        ...currentSplit.slice(0, lineStart + 1),
                        ...newSplit,
                        ...currentSplit.slice(lineEnd),
                    ].join("\n")
                })
            },
        }
    }
}

export abstract class Renderer {
    abstract render(renderHost: RenderHost, codeBlock: CodeBlockContext): void
    abstract destroy(renderHost: RenderHost): void
}

export interface CodeBlockContext {
    /**
     * Returns the entire codeblock, including the codeblock fence.
     */
    read(): string
    /**
     * Returns the codeblock content, excluding the codeblock fence.
     */
    readContent(): string
    /**
     * Replaces the entire codeblock, not including the codeblock fence.
     */
    writeContent(newContent: string): Promise<void>
}
