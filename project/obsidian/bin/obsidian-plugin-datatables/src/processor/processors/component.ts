/*
import { FunctionComponent } from "react"

import { BlockProcessor } from "../processor.abstract"
import { MarkdownProcessor } from "../processor.decorators"
import { ReactRenderer } from "../renderers/react.renderer"
import { RenderContext } from "../renderers/renderer"

@MarkdownProcessor("obby-component")
export class ComponentProcessor extends BlockProcessor {
    constructor(private reactRenderer: ReactRenderer) {
        super()
    }

    override getRenderContext(source: string): RenderContext<unknown> {
        return this.reactRenderer.getAsyncRenderContext(() => {
            // https://esbuild.github.io/content-types/#direct-eval
            const component = new Function("props", source) as FunctionComponent

            return {
                component,
                props: {},
            }
        })
    }
}
*/
