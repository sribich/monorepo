import { dirname, join } from "path"
import { fileURLToPath } from "url"
import { Worker } from "worker_threads"

import type { Plugin } from "../plugin.js"
import { diagnostics } from "../diagnostics/diagnostics.plugin.js"

export const declarationPlugin = (): Plugin => {
    return async (config) => {
        const workerTerminationCallbacks: (() => void)[] = []

        if (process.env["SKIP_DECLARATIONS"]) {
            return null
        }

        // if (!config.build.formats.includes("f")) {
        //     return null
        // }

        return {
            terminate() {
                for (const callback of workerTerminationCallbacks) {
                    callback()
                }
            },
            esbuild: {
                name: "tsbuild:declaration",
                async setup(build) {
                    // It's hacky, but we need to call entrypoints here since plugins are initialised
                    // before the esbuild context.
                    const entrypoints = config.config.entrypoints

                    const format = build.initialOptions.format
                    const outdir = build.initialOptions.outdir

                    if (!outdir || !format || (format !== "cjs" && format !== "esm")) {
                        return
                    }

                    const worker = new Worker(
                        join(dirname(fileURLToPath(import.meta.url)), "worker.js"),
                        {
                            workerData: {
                                config: {
                                    build: config.build,
                                },
                                format,
                                outdir,
                                entrypoints,
                            },
                        },
                    )

                    const resolvedPromise = Promise.resolve()
                    let currentPromise = resolvedPromise

                    worker.on("message", (message) => {
                        switch (message) {
                            case "ready":
                                return
                            case "emitComplete":
                                return
                            default:
                                throw new Error(`Unknown message ${message}`)
                        }
                    })

                    worker.on("exit", async (e) => {})

                    workerTerminationCallbacks.push(() => {
                        worker.postMessage("exit")
                    })

                    let cancellationToken

                    build.onStart(
                        diagnostics.span(
                            { name: "plugin:declaration", phase: "onStart" },
                            async () => {
                                worker.postMessage("emit")

                                currentPromise = new Promise((resolve, reject) => {
                                    const handler = (message: string) => {
                                        if (message === "emitComplete") {
                                            worker.off("message", handler)
                                            resolve()
                                        }
                                    }
                                    worker.on("message", handler)
                                })
                            },
                        ),
                    )

                    build.onEnd(
                        diagnostics.span(
                            { name: "plugin:declaration", phase: "onEnd" },
                            async () => {
                                await currentPromise
                                currentPromise = resolvedPromise
                            },
                        ),
                    )
                },
            },
        }
    }
}
