import { writeFile } from "node:fs/promises"

import type { BuildOptions, BuildContext as EsbuildBuildContext } from "esbuild"
import esbuild from "esbuild"

import type { RunnerContext } from "../../context/runner.js"
import { logger } from "../../logger.js"
import { Backend } from "../backend.js"
import { gatherEntrypoints } from "./entrypoint-resolver.js"
import { getDefaultOptions } from "./options.js"

/**
 * TODO: Circular dependency checking
 * @see https://www.npmjs.com/package/circular-dependency-plugin
 * @see https://github.com/pahen/madge
 * @see https://github.com/evanw/esbuild/issues/667
 */
export class EsbuildBackend extends Backend {
    private build: EsbuildBuildContext<BuildOptions> | undefined

    private initialOptions: BuildOptions
    private master: boolean

    constructor(initialOptions: BuildOptions, context: RunnerContext, master = false) {
        super(context)

        this.initialOptions = initialOptions
        this.master = master
    }

    async initialise(): Promise<void> {
        const buildContext = this.context.build

        // const options = await this.pluginContext.modifyConfig({
        const options: BuildOptions = {
            ...getDefaultOptions(this.context, this.master, this.initialOptions.format),
            ...(this.context.config.esbuild ?? {}),
            ...this.initialOptions,
            outdir:
                this.context.config.formats.length > 1
                    ? `${this.context.config.outdir}/${this.initialOptions.format}`
                    : `${this.context.config.outdir}`,

            conditions: [
                // "react-server"
            ],
            define: {
                "process.env.NODE_ENV":
                    buildContext.mode === "build" ? '"production"' : '"development"',
            },
        }

        if (buildContext.mode === "dev") {
            options.conditions ??= []
            options.conditions.unshift("development")
        }

        // We need to collect both the sole entrypoint that might be used when
        // we're bundling code, but also all possible entrypoint to feed into
        // typescript for declaration emit.
        const [maybeBundledEntrypoints, genericEntrypoints] = await gatherEntrypoints(
            options,
            this.context,
        )

        // TODO: Not sure about this guy
        // if (!this.context.has(EntrypointContext)) {
        //     this.context.add(new EntrypointContext(genericEntrypoints))
        // }

        options.entryPoints = [
            ...((options.entryPoints as string[]) ?? []),
            ...maybeBundledEntrypoints,
        ]

        this.build = await esbuild.context(options)
    }

    async terminate(): Promise<void> {
        await this.build?.dispose()
        this.build = undefined
    }

    async compile(): Promise<void> {
        try {
            await this.build?.rebuild()

            // TODO: This should probably actually be a plugin. cjs-compat?
            //       In the plugin we can check if outdir ends with cjs instead of checking
            //       formats length.
            if (this.initialOptions.format === "cjs" && this.context.config.formats.length > 1) {
                const filePath = `${this.context.config.outdir}/${this.initialOptions.format}/package.json`

                await writeFile(filePath, '{ "type": "commonjs" }')
            }
        } catch (e) {
            if (e instanceof Error) {
                logger.error("build", e.message)
            }
        }
    }
}
