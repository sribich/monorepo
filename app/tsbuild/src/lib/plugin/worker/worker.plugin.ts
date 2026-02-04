import esbuild, { type BuildOptions } from "esbuild"

import type { Plugin } from "../plugin.js"

export const workerPlugin = (): Plugin =>  (context) => {
    const pluginContext = context.plugin

    return {
        esbuild: {
            name: "tsbuild:inline-worker",
            setup(build) {
                build.onLoad({ filter: /\.worker\.?(js|jsx|ts|tsx)?/ }, async ({ path }) => {
                    const workerCode = await buildWorker(path, {
                        ...build.initialOptions,
                        plugins: pluginContext.getSubbuildPlugins(),
                    })

                    return {
                        contents: [
                            "import inlineWorker from '__inline-worker'",
                            "export default function Worker() {",
                            `  return inlineWorker(${JSON.stringify(
                                new TextDecoder().decode(workerCode),
                            )})`,
                            "}",
                        ].join("\n"),
                        loader: "js",
                    }
                })

                const inlineWorkerFunctionCode = [
                    "export default function inlineWorker(scriptText) {",
                    "  let blob = new Blob([scriptText], {type: 'text/javascript'})",
                    "  let url = URL.createObjectURL(blob)",
                    "  let worker = new Worker(url)",
                    "  URL.revokeObjectURL(url)",
                    "  return worker",
                    "}",
                ].join("\n")

                build.onResolve({ filter: /^__inline-worker$/ }, ({ path }) => {
                    return { path, namespace: "inline-worker" }
                })

                build.onLoad({ filter: /.*/, namespace: "inline-worker" }, () => {
                    return { contents: inlineWorkerFunctionCode, loader: "js" }
                })
            },
        },
    }
}

async function buildWorker(workerPath: string, baseOptions: BuildOptions) {
    const { outdir, ...options } = baseOptions

    const result = await esbuild.build({
        ...options,
        entryPoints: [workerPath],
        bundle: true,
        minify: true,
        outfile: "__internal_do_not_write__.js",
        write: false,
        legalComments: "none",
        sourcemap: false,
    })

    return result.outputFiles[0]?.contents
}
