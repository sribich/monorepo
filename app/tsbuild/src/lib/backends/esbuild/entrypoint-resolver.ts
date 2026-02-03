import { join, resolve } from "node:path"
import esbuild from "esbuild"
import type { BuildOptions } from "esbuild"
import type { RunnerContext } from "../../context/runner.js"

/**
 * TODO: We're gathering entrypoints twice. We should create a global
 *       to handle entrypoints that consumers can listen to and update
 *       from when entrypoints change.
 */
export const gatherEntrypoints = async (
    buildOptions: BuildOptions,
    context: RunnerContext,
): Promise<[Set<string>, Set<string>]> => {
    const config = context.config
    const repository = context.repository

    const initialEntrypoints = config.entrypoints

    const clonedOptions = {
        ...buildOptions,
        entryPoints: [...initialEntrypoints],
        bundle: true,
        metafile: true,
        write: false,
        plugins: [
            {
                name: "tsbuild:gather-entrypoints",
                setup(build) {
                    build.onResolve({ filter: /.*/ }, (args) => {
                        if (!args.path.startsWith(".") && !initialEntrypoints.includes(args.path)) {
                            return { external: true }
                        }
                        return
                    })
                },
            },
        ],
    } satisfies BuildOptions

    const build = await esbuild.context(clonedOptions)
    const result = await build.rebuild()

    const inputs = Object.keys(result?.metafile?.inputs ?? {})
    const entryPoints = inputs
        .filter((it) => !it.includes("node_modules"))
        .filter((it) => {
            const projectRoot = repository.project.path
            const entryPointPath = join(repository.project.path, it)

            return resolve(entryPointPath).startsWith(projectRoot + "/")
        })

    await build.dispose()

    if (config.bundle) {
        return [new Set(initialEntrypoints), new Set(entryPoints)]
    }

    return [new Set(entryPoints), new Set(entryPoints)]
}
