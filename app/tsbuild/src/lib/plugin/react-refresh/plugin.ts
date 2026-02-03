import { readFile } from "node:fs/promises"
import { dirname, join, relative } from "node:path"
import { fileURLToPath } from "node:url"

import { type ServerType, serve } from "@hono/node-server"
import { createNodeWebSocket } from "@hono/node-ws"
import { Hono } from "hono"
import assert from "node:assert"
import { readFileSync } from "node:fs"
import { cwd } from "node:process"
import type { Metafile } from "esbuild"
import type { WSContext } from "hono/ws"
import type { Plugin } from "../plugin.js"
import { addToHead, createScriptElement, createTextNode } from "../../builtins/html/plugin.js"

const __dirname = dirname(fileURLToPath(import.meta.url))

const VIRTUAL_MODULE_ID = "virtual:react-refresh"
const RESOLVED_VIRTUAL_MODULE_ID = `\0${VIRTUAL_MODULE_ID}`

const IS_FAST_REFRESH_ENABLED = /\$RefreshSig\$\(/

const PREAMBLE_CODE = `
import * as RefreshRuntime from "react-refresh/runtime";
import { injectIntoGlobalHook } from "react-refresh/runtime";

injectIntoGlobalHook(window);

window.$RefreshReg$ = () => {};
window.$RefreshSig$ = () => (type) => type;
window.RefreshRuntime = RefreshRuntime;
`

export const reactRefreshPlugin = (): Plugin => {
    return async (context) => {
        const project = await context.repository.getCurrentProject()

        if (!project.dependencies.includes("react")) {
            return null
        }

        if (project.name === "@railgun/pix") {
            return null
        }

        const runtimeCode = (await readFile(join(__dirname, "./hmr/runtime.js"))).toString()
        const prefixCode = (await readFile(join(__dirname, "./hmr/prefix.js"))).toString()

        return {
            modifyConfig(config) {
                /*
                if (!options.entryPoints) {
                    options.entryPoints = [join(__dirname, "./hmr/entry.js")]
                    options.entryPoints.push("react", "react-dom", "react-refresh/runtime")
                }
                if (Array.isArray(options.entryPoints)) {
                    options.entryPoints.push(join(__dirname, "./hmr/entry.js") as never)
                    options.entryPoints.push(
                        "react" as never,
                        "react-dom" as never,
                        "react-refresh/runtime" as never,
                    )
                }
                options.assetNames = "[name]-[hash]"
                options.chunkNames = "[name]-[hash]"
                options.entryNames = "[name]-[hash]"
                options.bundle = true
                options.splitting = true
                options.supported = {
                    "import-meta": true,
                }
                options.format = "esm"
                */

                config.rolldown ??= {}
                config.rolldown.input ??= {}
                config.rolldown.input.transform ??= {}
                config.rolldown.input.transform.jsx ??= {}

                if (typeof config.rolldown.input.transform.jsx === "string") {
                    throw new Error("Cannot use string 'transform.jsx' values with react-refresh.")
                }

                config.rolldown.input.transform.jsx = {
                    ...config.rolldown.input.transform.jsx,
                    runtime: "automatic",
                    refresh: true,
                    development: true,
                }
            },
            transformIndexHtml(document) {
                const script = createScriptElement({
                    path: "",
                    children: [],
                })

                script.childNodes = [createTextNode(script, PREAMBLE_CODE)]

                addToHead(document, script)
            },
            rolldown: {
                name: "",
                resolveId(source) {
                    if (source === VIRTUAL_MODULE_ID) {
                        return RESOLVED_VIRTUAL_MODULE_ID
                    }

                    return
                },
                load(id) {
                    if (id === RESOLVED_VIRTUAL_MODULE_ID) {
                        return PREAMBLE_CODE
                    }

                    return
                },
                renderChunk(code, chunk) {
                    const id = chunk.fileName

                    if (!id || !IS_FAST_REFRESH_ENABLED.test(code)) {
                        return { code }
                    }

                    const hmrId = JSON.stringify(relative(cwd(), id))
                    const newCode = `
let prevRefreshReg = window.$RefreshReg$;
let prevRefreshSig = window.$RefreshSig$;

window.$RefreshReg$ = window.RefreshRuntime.getRefreshReg(${hmrId});
window.$RefreshSig$ = window.RefreshRuntime.createSignatureFunctionForTransform;

${code}

window.$RefreshReg$ = prevRefreshReg;
window.$RefreshSig$ = prevRefreshSig;

if (import.meta.hot) {
    window.RefreshRuntime.__hmr_import(import.meta.url).then((currentExports) => {
        window.RefreshRuntime.registerExportsForReactRefresh(${hmrId}, currentExports);

        import.meta.hot.accept((nextExports) => {
            if (!nextExports) {
                return;
            }

            const invalidateMessage = window.RefreshRuntime.validateRefreshBoundaryAndEnqueueUpdate(${hmrId}, currentExports, nextExports);

            if (invalidateMessage) {
                import.meta.hot.invalidate(invalidateMessage);
            }
        });
    });
}
`

                    return { code: newCode }
                },
            },

            /*
            esbuild: {
                name: "react-refresh",
                setup: async (build) => {


                    if (buildContext.mode !== "dev") {
                        return
                    }

                    const app = new Hono({})

                    const { injectWebSocket, upgradeWebSocket } = createNodeWebSocket({ app })

                    const sockets = [] as WSContext[]

                    app.get(
                        "/__hmr__",
                        upgradeWebSocket((c) => ({
                            onOpen(_, conn) {
                                sockets.push(conn)
                            },
                            onClose(_, conn) {
                                const idx = sockets.findIndex((it) => it === conn)

                                if (idx !== -1) {
                                    sockets.splice(idx, 1)
                                }
                            },
                        })),
                    )

                    const server = await new Promise<ServerType>((resolve) => {
                        const server = serve({ fetch: app.fetch, port: 9094 }, () =>
                            resolve(server),
                        )
                    })

                    injectWebSocket(server)

                    let lastManifest: Metafile | undefined = undefined

                    build.onEnd((manifest) => {
                        if (!lastManifest) {
                            lastManifest = manifest.metafile
                            return
                        }

                        const lastInputsSet = new Set(Object.keys(lastManifest.inputs))
                        const lastInputToOutput = Object.entries(lastManifest.outputs).reduce(
                            (acc, [outputFile, output]) => {
                                Object.keys(output.inputs).forEach((input) => {
                                    if (lastInputsSet.has(input)) {
                                        // @ts-ignore
                                        acc[input] = outputFile
                                    }
                                })
                                return acc
                            },
                            {},
                        )

                        assert(manifest.metafile)
                        const newInputsSet = new Set(Object.keys(manifest.metafile.inputs))
                        const newInputToOutput = Object.entries(manifest.metafile.outputs).reduce(
                            (acc, [outputFile, output]) => {
                                Object.keys(output.inputs).forEach((input) => {
                                    if (newInputsSet.has(input)) {
                                        // @ts-ignore
                                        acc[input] = outputFile
                                    }
                                })
                                return acc
                            },
                            {},
                        )

                        const updates = Object.keys(manifest.metafile.inputs).reduce(
                            (acc, input) => {
                                // @ts-ignore
                                if (lastInputToOutput[input] !== newInputToOutput[input]) {
                                    // @ts-ignore
                                    acc.push({
                                        type: "update",
                                        id: input,
                                        // @ts-ignore
                                        url: "/" + newInputToOutput[input]?.replace(/^dist\//, ""),
                                    })
                                }

                                return acc
                            },
                            [],
                        )

                        lastManifest = manifest.metafile

                        const message = JSON.stringify({ type: "hmr", updates })

                        for (const socket of sockets) {
                            socket.send(message)
                        }
                    })
                },
            },
             */
        }
    }
}
