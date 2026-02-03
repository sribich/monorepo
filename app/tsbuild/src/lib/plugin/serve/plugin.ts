import type { PluginBuild } from "esbuild"
import type { Plugin } from "../plugin.js"
import type { ServePluginConfig } from "./config.js"
import { AppServer } from "./servers/app.server.js"
import { WebServer } from "./servers/web.server.js"
import type { Immutable } from "../../util/immutable.js"
import { ArkErrors, scope, type Type } from "arktype"
import { logger } from "../../logger.js"
import type { IncomingMessage } from "node:http"
import { exit } from "node:process"
import { RunnerMode } from "../../runner.js"

const PLUGIN_NAME = "TSBUILD:SERVE"

const servePluginConfigParser = scope({
    proxy: {
        target: "string",
        "bypass?": "Function" as never as Type<(req: IncomingMessage) => boolean>,
    },
    plugin: {
        "host?": "string",
        "port?": "number",
        "proxy?": "Record<string, proxy>",
        "entrypoint?": "string",
        "reload?": "boolean",
        "nodeArgs?": "string[]",
    },
}).export().plugin

export const servePlugin = (config: ServePluginConfig = {}): Plugin => {
    const options = servePluginConfigParser(config)

    if (options instanceof ArkErrors) {
        logger.error(new Error(options.summary))
        exit(1)
    }

    return async (context) => {
        const buildContext = context.build

        // TODO: We should change these to something like `BuildKind`
        if (context.config.preset === "lib") {
            return null
        }

        if (buildContext.mode !== RunnerMode.DEV) {
            return null
        }

        if (
            context.config.entrypoints.length > 1 &&
            !options.entrypoint &&
            context.config.preset !== "webApp"
        ) {
            throw new Error(
                "Cannot have more than 1 entrypoint when serving. TODO: Add entrypoint specifier",
            )
        }

        /*
        if (config.build.formats.length > 1) {
            throw new Error("Cannot use --serve with more than 1 output format.")
        }
        */

        const server =
            context.config.preset === "nodeApp"
                ? new AppServer(options, context)
                : new WebServer(options, context)

        return {
            esbuild: (isMaster) => {
                if (!isMaster) {
                    return undefined
                }

                return {
                    name: PLUGIN_NAME,
                    setup(build) {
                        const entryPoint = getEntrypoint(build, options)

                        if (!entryPoint) {
                            return
                        }

                        build.onStart(async () => {
                            server.acquireLock()
                        })

                        build.onEnd(async (result) => {
                            server.releaseLock()

                            await server.start(/*result, entryPoint*/)
                        })
                    },
                }
            },
            rolldown: () => {
                return {
                    name: PLUGIN_NAME,
                    buildStart: () => {
                        server.acquireLock()
                    },
                    buildEnd: () => {
                        server.releaseLock()
                    },
                    writeBundle: async (outputOptions, outputBundle) => {
                        await server.start(/*result, outputBundle.fileName*/)
                    },
                }
            },
        }
    }
}

const getEntrypoint = (build: PluginBuild, config: Immutable<ServePluginConfig>) => {
    if (config.entrypoint) {
        return config.entrypoint
    }

    const entryPoints = build.initialOptions.entryPoints
    let entryPointArr = [] as string[]

    if (Array.isArray(entryPoints)) {
        if (entryPoints.length && typeof entryPoints[0] === "object") {
            throw new Error("Object array not supported")
        }

        entryPointArr = entryPointArr.concat(entryPoints as string[])
    } else if (entryPoints && typeof entryPoints === "object") {
        entryPointArr = entryPointArr.concat(Object.values(entryPoints))
    }

    if (entryPointArr.length === 0) {
        return
    }

    return entryPointArr[0]
}
