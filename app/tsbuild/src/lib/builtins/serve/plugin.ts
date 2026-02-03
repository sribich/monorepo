import { exit } from "node:process"
import type { Plugin } from "../../plugin/plugin.js"
import { RunnerMode } from "../../runner.js"
import { AppServer } from "./app_server.js"
import { WebServer } from "./web_server.js"

const PLUGIN_NAME = "tsbuild:serve"

export const servePlugin: Plugin =
    async (context) => {
        // config: ServePluginConfig = {}
        const config = context.config.server ?? {}

        if (context.build.mode !== RunnerMode.DEV) {
            return null
        }

        const assetMap = new Map<string, string>()

        const server = context.config.entrypoints[0]?.endsWith(".html")
            ? new WebServer(config, context, assetMap)
            : new AppServer(config, context)

        server.start()

        return {
            rolldown: {
                name: PLUGIN_NAME,
                generateBundle(_, bundle) {
                    for (const [id, entry] of Object.entries(bundle)) {
                        if (entry.type === "asset") {
                            assetMap.set(id, entry.source.toString())
                        } else {
                            assetMap.set(id, entry.code)
                        }
                    }
                },
            },
        }
    }
