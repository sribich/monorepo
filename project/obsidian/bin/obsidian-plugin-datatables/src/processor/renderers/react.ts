import type { FunctionComponent } from "react"
import { createRoot, type Root } from "react-dom/client"

import { Mount, MountError } from "../../ui/Mount"
import { type CodeBlockContext, Renderer, type RenderHost } from "../render"

export interface MountContext<
    TProps extends Record<string, unknown>,
    TContext extends Record<string, unknown>,
> {
    mount: { component: FunctionComponent<TProps>; props: TProps }
    context: (renderHost: RenderHost, codeBlock: CodeBlockContext) => TContext | Promise<TContext>
}

export class ReactRenderer<
    TProps extends Record<string, unknown>,
    TContext extends Record<string, unknown>,
> extends Renderer {
    private root?: Root | undefined
    private mountContext: MountContext<TProps, TContext>

    constructor(mountContext: MountContext<TProps, TContext>) {
        super()

        this.mountContext = mountContext
    }

    public destroy(): void {
        this.root?.unmount()
    }

    public async render(renderHost: RenderHost, codeBlock: CodeBlockContext): Promise<void> {
        this.root ??= createRoot(renderHost.processor.element)

        try {
            const context = await this.mountContext.context(renderHost, codeBlock)

            this.internalRender(this.root, context)
        } catch (e) {
            if (e instanceof Error) {
                this.renderError(this.root, e)
                return
            }

            this.renderError(this.root, new Error(`Failed to render component`))
        }
    }

    private internalRender(root: Root, context: TContext): void {
        const { mount } = this.mountContext

        root.render(
            Mount({
                component: mount.component,
                componentProps: mount.props,
                context,
            }),
        )
    }

    private renderError(root: Root, error: Error): void {
        root.render(MountError({ error }))
    }
}
